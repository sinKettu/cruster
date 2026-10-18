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

MCP starts in read-only/passive mode. The following operations are denied by default:

- sending HTTP requests;
- creating, editing, or deleting saved repeaters;
- running active Python audit rules;
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

Поля: `id`, `include_bodies` (по умолчанию `true`). Возвращает `name`, текстовый `request`, последний текстовый `response` и `parameters`: `address`, `https`, `redirects`, `max_redirects`. При `include_bodies=false` возвращаются заголовки запроса, а ответ опускается.

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

Поле: `id`. Удаляет репитер, возвращает `deleted_id` и число `remaining`. Последующие ID перенумеровываются.

### Ограничения репитера

- Сохранённые CLI/TUI-репитеры поддерживают текстовые UTF-8 запросы и заголовки. Для бинарного тела используйте `http_request_send` с `body_base64`; бинарные ответы доступны в `structured_response.body`.
- `raw_request` — структурированный HTTP-запрос, а не произвольная последовательность TCP-байтов: транспорт может нормализовать URL, заголовки и framing. Точное воспроизведение некорректного HTTP framing, HTTP/3, WebSocket-сеансы и CONNECT-туннели этим интерфейсом не предоставляются.
- MCP сериализует доступ к репитерам внутри процесса и заменяет файл атомарно. Одновременная запись в тот же проект из другого MCP-процесса, CLI или TUI не координируется; используйте одного писателя.

## Limits

- `--max-result-bytes <BYTES>` controls the maximum serialized tool result. Default: 1 MiB; accepted range: 1 KiB through 16 MiB.
- `--request-timeout-seconds <SECONDS>` controls direct-send timeout. Default: 30 seconds; accepted range: 1 through 600 seconds.
- History list pages are capped at 100 records.
- MCP inputs never accept arbitrary filesystem paths. History and repeaters use only the selected project's `http.jsonl` and `repeater.jsonl`; rules come only from the resolved audit configuration.

If a result exceeds the configured limit, request fewer records or omit bodies.

## Current scope

MCP предоставляет просмотр проекта, Python-аудит, редактирование и повторную отправку запросов, полный цикл работы с сохранёнными репитерами и чтение больших результатов по частям. Управление жизненным циклом уже запущенного интерактивного прокси, изменение захваченной истории и произвольные файловые/командные операции не предоставляются.
