# Model Context Protocol server

Cruster can run as a local MCP server over standard input/output. This allows an MCP-compatible AI agent to inspect a persisted Cruster project, analyze captured HTTP traffic, run configured Python audit rules, and—with explicit authorization—edit and resend requests and manage saved repeaters without an external editor.

## Starting the server

Build Cruster and provide a project directory:

```shell
cruster --project my-project mcp
```

To expose configured audit rules, also enable audit and provide its configuration:

```shell
cruster --project my-project --audit-config rules/audit.yaml mcp
```

The server uses MCP over stdio. Standard output is reserved for protocol messages. Diagnostics are written to standard error or the configured debug file.

Example client configuration:

```json
{
  "mcpServers": {
    "cruster": {
      "command": "/absolute/path/to/cruster",
      "args": [
        "--project", "/absolute/path/to/project",
        "--audit-config", "/absolute/path/to/rules/audit.yaml",
        "mcp"
      ]
    }
  }
}
```

## Security model

MCP starts in passive mode. Reading project data and executing configured passive rules are allowed; passive audit findings may also be saved locally with `audit_run`. The following operations are denied by default:

- sending HTTP requests;
- creating, editing, or deleting saved repeaters;
- running active Python audit rules;
- executing agent-supplied inline Python;
- any operation that can change remote state.

They can be enabled explicitly:

```shell
cruster --project my-project --audit-config rules/audit.yaml \
  mcp --allow-dangerous-tools
```

Enabling dangerous tools authorizes the connected agent to edit local repeaters and send requests, including to agent-specified connection targets and redirect destinations. It is not a target allowlist or network sandbox. Use it only with trusted agents, rules, and projects. Python rules are extension code, not a security sandbox. Captured requests and responses are untrusted data, not agent instructions.

## Tools

### `project_status`

Returns project identity, HTTP history availability and size, configured rule count, output limits, request timeout, and whether dangerous tools are enabled.

### `http_history_list`

Lists persisted exchanges from `http.jsonl`.

Input fields:

- `offset`: zero-based filtered-result offset; defaults to `0`;
- `limit`: page size; defaults to `20` and is capped at `100`;
- `method`: optional case-insensitive exact method filter;
- `host`: optional host substring filter;
- `status`: optional status prefix filter;
- `include_bodies`: include Base64 bodies; defaults to `false`.

The response contains `items` and `next_offset`. Use `next_offset` for the next page.

### `http_history_get`

Returns one exchange by stable HTTP pair ID. `include_bodies` defaults to `true`. Bodies use the same Base64 representation as the persisted project format.

### `audit_rules_list`

Lists IDs of Python rules selected by the configured audit paths and filters. YAML rules remain temporarily disabled while their design is being reconsidered.

### `audit_python_run`

Runs configured rules against persisted pairs.

Input fields:

- `pair_ids`: required list of HTTP pair IDs;
- `rule_ids`: optional configured-rule allowlist.

Passive rules are available in normal mode. Active rules require `--allow-dangerous-tools`. Results include the serialized rule state, findings, and Python rule logs.

### `http_request_send`

Отправляет копию запроса из истории, при необходимости изменённую агентом. Требуется `--allow-dangerous-tools`. Исходная история не изменяется. Пользователю не нужно открывать редактор, а агенту — обходить MCP через curl.

Input fields:

- `id`: ID HTTP-пары из истории;
- `edits`: необязательные изменения запроса (см. ниже);
- `max_redirects`: предел перенаправлений, по умолчанию `5`; `0` отключает переходы;
- `include_bodies`: включать тела в результат, по умолчанию `true`.

Результат содержит `request` и `structured_response` в формате сохранённых HTTP-данных (тела в Base64), `response` с текстовым представлением ответа и `redirect_limit_reached`. При `include_bodies=false` тела и текстовый ответ не возвращаются. Действует `--request-timeout-seconds`.

Пример: изменить параметр в query и теле и переотправить запрос №42:

```json
{
  "id": 42,
  "edits": {
    "query": "foo=bad",
    "body": "foo=bad"
  }
}
```

Для изменения только query или только тела достаточно передать соответствующее поле.

### Формат `edits`

Все поля необязательны; отсутствующие значения сохраняются:

- `raw_request`: полная замена текстового запроса, включая первую строку, заголовки, пустую строку и тело. Первая строка использует origin-form: `POST /path?query HTTP/1.1`. Адрес подключения берётся из исходного запроса или `url`;
- `method`, `version`: HTTP-метод и версия;
- `url`: абсолютный HTTP(S) URL подключения; логин, пароль и fragment не допускаются;
- `path`: путь, без query;
- `query`: полная query-строка без `?`; пустая строка удаляет query;
- `headers`: полная замена заголовков массивом `{"name":"…","value":"…"}`. Повторяющиеся имена сохраняются; пустой массив удаляет все заголовки;
- `remove_headers`: список имён удаляемых заголовков без учёта регистра;
- `set_headers`: заменить все значения перечисленных заголовков; повторяющиеся имена в массиве сохраняются;
- `body`: точное UTF-8 тело, включая пустые строки и завершающие пробелы; пустая строка очищает тело;
- `body_base64`: точное бинарное тело в Base64; нельзя передавать одновременно с `body`.

Порядок применения: `raw_request`, затем `url`, `path`, `query`, полная замена заголовков, удаление заголовков, установка заголовков, тело. Остальные структурированные поля также переопределяют значения из `raw_request`. Неизвестные поля отклоняются, чтобы опечатки не приводили к незаметной отправке исходного запроса.

Адрес подключения и заголовок Host независимы: изменение `url` не заменяет Host. Для смены обоих нужно также передать `set_headers`. Content-Length рассчитывается транспортом по итоговому телу. HTTP/2 использует общий отправитель Cruster, включая преобразование Host в `:authority` и фильтрацию несовместимых заголовков.

### Сохранённые репитеры

MCP использует тот же файл `repeater.jsonl`, что CLI/TUI. Запросы возвращаются агенту как обычный текст; Base64 представления файла обрабатываются сервером. Все операции записи и отправки требуют `--allow-dangerous-tools`.

ID репитера — **позиция, начиная с 1**, как в CLI, а не ID пары из истории. После удаления последующие ID сдвигаются; перед дальнейшими действиями обновите список. Изменение истории HTTP не производится.

#### `repeater_list`

Поля: `offset` (по умолчанию `0`), `limit` (по умолчанию `20`, максимум `100`). Возвращает `items` с ID, именем, параметрами и признаком сохранённого ответа, а также `next_offset`. Тела не включаются.

#### `repeater_get`

Поля: `id` или `name`, `include_bodies` (по умолчанию `true`). Возвращает `name`, текстовый `request`, последний текстовый `response` и `parameters`: `address`, `https`, `redirects`, `max_redirects`. При `include_bodies=false` возвращаются только заголовки запроса и ответа.

#### `repeater_create`

Создаёт репитер из `history_id` или из `raw_request`. Для нового запроса без истории обязательны `changes.address` и `changes.https`. `raw_request` также может заменить запрос, скопированный из истории. Необязательный объект `changes` позволяет сразу изменить имя, запрос и параметры. Возвращается созданный репитер; отправка не выполняется.

```json
{
  "history_id": 42,
  "changes": {
    "name": "Проверка foo",
    "edits": {"query": "foo=bad", "body": "foo=bad"}
  }
}
```

#### `repeater_update`

Поля: `id`, `changes`. Объект `changes` принимает:

- `name`;
- `edits` в описанном выше формате;
- `address`: хост/IP подключения с необязательным портом, IPv6 — в квадратных скобках;
- `https`: использовать TLS;
- `redirects`: следовать перенаправлениям;
- `max_redirects`: предел перенаправлений.

Отсутствующие параметры сохраняются. Настройки адреса применяются перед `edits`, поэтому явно переданный `edits.url` имеет приоритет. Изменение сбрасывает старый ответ, но не отправляет запрос.

#### `repeater_send`

Поля: `id`, необязательные `changes`, `include_bodies` (по умолчанию `true`). Применяет и сохраняет изменения, отправляет запрос и сохраняет последний ответ. Возвращает те же данные, что `http_request_send`, плюс `repeater_id`.

```json
{
  "id": 1,
  "changes": {"edits": {"body": "foo=bad"}},
  "include_bodies": false
}
```

Изменения сохраняются до сетевого вызова; ответ — до проверки ограничения размера результата. Если сеть завершилась ошибкой или тайм-аутом, запрос уже мог достичь сервера: не повторяйте изменяющую состояние операцию автоматически. Если результат слишком большой, прочитайте сохранённый ответ, а не отправляйте запрос повторно.

#### `repeater_content_get`

Читает сохранённый запрос или ответ по частям, без сетевой отправки. Поля: `id`, `part` (`request` или `response`), `offset` (смещение в байтах, по умолчанию `0`), `limit` (по умолчанию 16384 байта, максимум 65536, автоматически уменьшается под лимит результата).

Возвращает `data_base64` с точными байтами фрагмента, `text` (если фрагмент целиком является UTF-8), `total_bytes`, `offset` и `next_offset`. Продолжайте чтение с `next_offset`, пока оно не станет `null`. Это позволяет агенту анализировать большие ответы без ручного открытия файлов.

#### `repeater_delete`

Поля выбора: `id` **или** `name`. Удаляет репитер, возвращает `deleted_id` и число `remaining`. Последующие ID перенумеровываются.

Во всех инструментах выбора сохранённого репитера (`repeater_get`, `repeater_update`, `repeater_send`, `repeater_delete`, `repeater_content_get`) вместо `id` допускается точное `name`. Нужно передать ровно одно поле выбора. При совпадающих именах выбирается первый репитер, как в CLI. Для переименования используйте `changes.name`.

При `repeater_get` с `include_bodies=false` сохраняются заголовки **и запроса, и ответа**, но тела исключаются.

## Соответствие команд и опций CLI

MCP предоставляет те же действия над данными, не запуская CLI как дочерний процесс. Названия и представление параметров адаптированы к структурированным вызовам инструментов.

### Общая реализация CLI и MCP

MCP использует операции CLI напрямую или общий код, выделенный из существующих CLI-обработчиков:

- [HTTP query](../src/cli/http/show.rs) — единые разбор диапазонов, выбор по позициям/ID, extraction и regex-фильтрация. Разбор диапазонов также используется при выборе пар для аудита.
- [Чтение истории](../src/cli/http.rs) — единая обработка JSONL-строк для CLI и MCP, включая незавершённую запись в режиме follow.
- [Операции репитера](../src/cli/repeater/operations.rs) — общий выбор по номеру/имени, создание из истории и цикл сохранения → отправки → сохранения ответа. [Чтение и атомарная запись](../src/cli/repeater.rs) используют единственный формат хранения CLI/TUI, без отдельной MCP-модели файла.
- [Исполнение аудита](../src/audit/execution.rs) — общий запуск существующих CLI workers и выполнение одного правила. [Операции аудита](../src/audit/operations.rs) объединяют фильтрацию пар и чтение/сортировку находок.

Только адаптация интерфейса остаётся раздельной: CLI печатает результат и при необходимости открывает редактор; MCP проверяет разрешения, возвращает JSON, ограничивает размер ответа и поддерживает пагинацию/курсоры. Изменение общего алгоритма выбора, исполнения или сохранения применяется к обоим интерфейсам.

| CLI | MCP | Опции и особенности |
| --- | --- | --- |
| `http show` | `http_history_query` | Все диапазоны INDEX, точный ID, regex, extraction, URL, summary, raw JSONL, pretty, исключение тел |
| `http follow` | `http_history_follow` | Новые записи с курсором, `no_old_lines`, URL; вместо бесконечного вывода — ограниченное ожидание |
| `repeater list` | `repeater_list` | Имена, параметры и пагинация |
| `repeater show` | `repeater_get` | Номер или имя, заголовки без тел |
| `repeater add` | `repeater_create` | ID захваченной пары; дополнительно можно создать новый запрос |
| `repeater edit` | `repeater_update` | Имя, адрес, HTTPS, redirects, max_redirects; дополнительно изменения самого запроса |
| `repeater exec` | `repeater_send` | Номер или имя, изменения перед отправкой, режим без тел. Внешний редактор не нужен |
| `audit run` | `audit_run` | Имя и сохранение, regex-фильтры HTTP, пути правил, include/exclude по ID и тегам, active/passive, подробности и параллелизм |
| `audit test` | `audit_run` без сохранения | Тестовый запуск настроенных правил на выбранной истории. Неиспользуемый CLI-аргумент ARG не переносится |
| Проверка одного правила | `audit_test` | Настроенное правило, файл или inline Python; сохранённая пара либо явно заданные request/response |
| `audit debug-rule` | `audit_debug_rule` | Правило, конкретная пара или вся история; состояния и Python-логи вместо терминального debug-вывода |
| `audit print` | `audit_results_list` | Одна находка или все, сортировка severity/rule ID, начальные данные, данные отправок, исключение тел/данных |
| `help`, `--help` | MCP `tools/list` | Описания инструментов и типизированные схемы параметров |

Не переносятся только способы взаимодействия с терминалом: цвета/ANSI, ширина таблиц, внешний редактор, глобальная настройка консольного логгера, бесконечный вывод и повторяющие команды короткие aliases. Опция `--force` не нужна: MCP не открывает редактор вообще. Данные, которые CLI показывает в этих режимах, доступны структурированно.

Глобальные параметры проекта, workplace, основного конфига и audit-конфига задаются при запуске MCP тем же Cruster. Опции прослушивания прокси, TLS-сертификатов и интерактивного TUI не являются командами CLI работы с сохранённым проектом и не добавляют управления работающим прокси.

### `http_history_query`

- `range`: CLI INDEX — `n` (первые n, по умолчанию `100`), `a` (все), `n-m` (позиции от 1, включительно), `-n` (последние n), `iN`/`Ni` (ID пары).
- `id`: точный ID независимо от диапазона. Как в CLI, точный ID включает pretty-представление; для структурированной пары используйте прежний `http_history_get`.
- `format`: `summary` (по умолчанию), `urls`, `structured`, `raw` или `pretty`. Pretty не содержит ANSI; raw возвращает исходные строки JSONL.
- `filter`: regex по первой строке, заголовкам и телу запроса/ответа через общий HTTP matcher. В raw-режиме regex применяется к исходной JSONL-строке, как в CLI.
- `extract`: `method=…`, `status=…`, `host=…`, `path=…`, с той же prefix-семантикой, что CLI.
- `include_bodies` (по умолчанию `true`), `no_body` (по умолчанию `false`). Raw несовместим с extraction и исключением тел.
- `offset`, `limit`: пагинация выбранного результата (по умолчанию 20, максимум 100); `content_offset` — продолжение одной большой записи.

Результат содержит `items`, `next_offset`, `next_content_offset`. Если одна запись не помещается, она представляется Base64-фрагментом её JSON-кодировки с полями `encoding`, `media_type`, `data`, `offset`, `total_bytes`. Передавайте оба возвращённых смещения, сохраняя остальные параметры. Соберите байты большой записи и затем декодируйте JSON; это сохраняет и Unicode, и бинарные тела в структурированном формате.

```json
{
  "range": "-50",
  "format": "urls",
  "extract": "status=4",
  "filter": "(?i)token|authorization",
  "limit": 20
}
```

### `http_history_follow`

Поля: `cursor` (байтовый курсор, не ID пары), `no_old_lines`, `urls`, `wait_ms` (по умолчанию 1000, максимум 30000), `limit` (20, максимум 100), `content_offset`.

Первый вызов без курсора читает историю с начала; `no_old_lines=true` начинает с конца завершённых записей. Ответ: `items`, `next_cursor`, `next_content_offset`, `has_more`. Для продолжения передайте `cursor=next_cursor` и `content_offset=next_content_offset`. Незавершённая последняя JSONL-строка ожидает дописывания и не теряется. Курсор нужно использовать только с тем же проектом и неусечённым файлом истории.

### `http_history_content_get`

Поля: `id`, `part` (`request`, `response`, `record`), `offset` в байтах, `length`. Для request/response возвращаются точные декодированные **байты тела**, для record — исходная JSONL-запись. Ответ содержит Base64 `data`, `total_bytes`, `next_offset`. Размер автоматически подстраивается под лимит результата.

### `audit_run`

- `selection.pair_ids`: ID HTTP-пар; `selection.range`: диапазон; `selection.filters`: дополнительные regex. Выбор ID пересекается с диапазоном и фильтрами. Несколько regex объединяются через OR и дополняют фильтры audit-конфига.
- `options.passive`, `options.active`: переопределение типов правил. Active требует разрешения опасных инструментов.
- `options.include`, `options.exclude`: объекты с массивами `ids`, `tags`, `paths` (regex по путям правил). Значения дополняют настроенные selectors; `options.reset_selectors=true` сначала очищает их.
- `options.rule_paths`: заменить пути загрузки правил; `options.add_paths`: добавить пути. Доступ разрешён внутри выбранного проекта и настроенных корней правил; выход за них, в том числе через symlink, отклоняется. Чтобы использовать другой каталог, укажите его в конфиге запуска MCP либо передайте правило inline в тестировании.
- `tasks`: параллелизм от 1 до 256, по умолчанию из audit-конфига/числа CPU.
- `verbose`: включать состояния выполнения и Python-логи, по умолчанию из конфига.
- `name`: имя запуска, по умолчанию из конфига; допустим безопасный basename без `audit_` и `.jsonl`.
- `persist`: сохранять находки в совместимый с CLI `audit_NAME.jsonl`, по умолчанию `false`. Для аналога CLI run передавайте `true`, для CLI test — `false`.

Результат содержит счётчики правил, пар, выполнений, находок, ошибок и пропусков, массив `entries` и число `omitted_entries`. Подробности, не поместившиеся в лимит, опускаются, но сохранённые находки остаются доступны через инструменты чтения. Существующие файлы результатов не перезаписываются. Прерванный запуск может оставить частичный файл с уже записанными находками.

```json
{
  "name": "selected",
  "persist": true,
  "selection": {"filters": ["^GET https?://example\\.com/"]},
  "options": {
    "passive": true,
    "active": false,
    "include": {"tags": ["technologies"]}
  },
  "tasks": 4,
  "verbose": true
}
```

### `audit_test` и `audit_debug_rule`

Поле `rule` выбирает правило:

- `{"source":"configured","id":"rule-id"}`;
- `{"source":"file","path":"rule.py"}` внутри проекта/настроенных корней;
- `{"source":"inline","python":"…"}` — код с обычными Python-метаданными Cruster. Требует разрешения опасных инструментов независимо от типа правила.

Поле `pair`:

- `{"source":"saved","id":42}`;
- `{"source":"raw","request":"GET / HTTP/1.1\r\nHost: example.test\r\n\r\n","url":"https://example.test/","response":"HTTP/1.1 200 OK\r\n\r\nbody"}`.

В raw-режиме вместо `response` можно явно передать `fetch_response=true`: запрос будет отправлен, что требует разрешения опасных инструментов. Неявных сетевых запросов нет.

Возвращаются `pair_id`, `rule_id`, `state`, `logs`. Для `audit_debug_rule` поле `pair` необязательно: без него правило проверяется на всех сохранённых парах с пагинацией `offset`/`limit`; ответ содержит `items`, `total`, `next_offset`, `output_truncated`. Повторный вызов на той же странице снова выполняет правило, поэтому для активных правил не повторяйте уже выполненные страницы автоматически.

Тайм-аут прямой HTTP-отправки применяется к `fetch_response`, но не является ограничением времени произвольного Python-кода: текущий общий исполнитель Python не поддерживает принудительную безопасную отмену. Выполняйте только доверенные правила.

### `audit_results_list`

Обязательное `name`; необязательные `id` находки, `pair_ids`, `rule_ids`, `severities`, regex `filter`. `sort`: `file` (по умолчанию), `rule_id`, `severity`, `severity_rule_id`. Сортировка выполняется до пагинации `offset`/`limit`.

`include_data` включает данные отправок/ответов находки, `include_initial` — исходную пару, `include_bodies` — тела. По умолчанию все три поля `false`. Это эквиваленты CLI no-data, initial-data и without-body без зависимости от форматирования вывода. Для каждой находки добавляется URL исходной пары. Возвращаются `items`, `total`, `next_offset`.

### `audit_result_content_get`

Поля: `name`, `id` находки, `offset`, `length`. Читает полную находку в JSON независимо от размера, возвращая UTF-8 `content`, `next_offset` и размер. Смещения измеряются в байтах; граница фрагмента не разрезает Unicode-символ.

### Ограничения репитера

- Сохранённые CLI/TUI-репитеры поддерживают текстовые UTF-8 запросы и заголовки. Для бинарного тела используйте `http_request_send` с `body_base64`; бинарные ответы доступны в `structured_response.body`.
- `raw_request` — структурированный HTTP-запрос, а не произвольная последовательность TCP-байтов: транспорт может нормализовать URL, заголовки и framing. Точное воспроизведение некорректного HTTP framing, HTTP/3, WebSocket-сеансы и CONNECT-туннели этим интерфейсом не предоставляются.
- MCP сериализует доступ к репитерам внутри процесса и заменяет файл атомарно. Одновременная запись в тот же проект из другого MCP-процесса, CLI или TUI не координируется; используйте одного писателя.

## Limits

- `--max-result-bytes <BYTES>` controls the maximum serialized tool result. Default: 1 MiB; accepted range: 1 KiB through 16 MiB.
- `--request-timeout-seconds <SECONDS>` controls direct-send timeout. Default: 30 seconds; accepted range: 1 through 600 seconds.
- History list pages are capped at 100 records.
- History and repeaters use only the selected project's `http.jsonl` and `repeater.jsonl`. Audit rule paths are confined to the project and configured rule roots; result names are validated basenames.

If a result exceeds the configured limit, request fewer records or omit bodies.

## Current scope

MCP предоставляет просмотр проекта, Python-аудит, редактирование и повторную отправку запросов, полный цикл работы с сохранёнными репитерами и чтение больших результатов по частям. Управление жизненным циклом уже запущенного интерактивного прокси, изменение захваченной истории и произвольные файловые/командные операции не предоставляются.
