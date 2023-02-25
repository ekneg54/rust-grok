# rustgrok

rustgrok is a python wrapper for the rust grok library written by Michael Nitchinger(https://github.com/daschl/grok).


## Installation

```
pip install rustgrok
```

## Usage

```python
grok = rustgrok.Grok()
grok.compile("%{IP:client} %{WORD:method} %{URIPATHPARAM:request} %{NUMBER:bytes} %{NUMBER:duration}")
result = grok.match_against("55.3.244.1 GET /index.html 15824 0.043")
assert result.get("client") == "55.3.244.1"
assert result.get("method") == "GET"
assert result.get("request") == "/index.html"
assert result.get("bytes") == "15824"
assert result.get("duration") == "0.043"
```
