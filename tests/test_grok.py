import rustgrok
class TestGrok:

    def test_init(self):
        grok = rustgrok.Grok("%{USERNAME}")
        result = grok.match_against("root")
        assert result.get("USERNAME") == "root"