# pylint: disable=no-member
# pylint: disable=missing-docstring
import pytest
import rustgrok


class TestGrok:

    def test_init_without_pattern(self):
        rustgrok.Grok()

    def test_init_with_pattern_raises(self):
        with pytest.raises(TypeError):
            rustgrok.Grok("pattern")

    def test_basic_usecase(self):
        grok = rustgrok.Grok()
        grok.compile("%{USERNAME}")
        result = grok.match_against("root")
        assert result.get("USERNAME") == "root"

    def test_multi_match(self):
        grok = rustgrok.Grok()
        grok.compile("%{IP:client} %{WORD:method} %{URIPATHPARAM:request} %{NUMBER:bytes} %{NUMBER:duration}")
        result = grok.match_against("55.3.244.1 GET /index.html 15824 0.043")
        assert result.get("client") == "55.3.244.1"
        assert result.get("method") == "GET"
        assert result.get("request") == "/index.html"
        assert result.get("bytes") == "15824"
        assert result.get("duration") == "0.043"

    def test_no_match_returns_empty_dict(self):
        grok = rustgrok.Grok()
        grok.compile("%{IP:client} %{WORD:method}")
        result = grok.match_against("bla bla")
        assert result == {}

    def test_add_custom_pattern(self):
        grok = rustgrok.Grok()
        grok.add_pattern("USER", "root")
        grok.compile("%{USER}")
        result = grok.match_against("root")
        assert result == {"USER": "root"}
