import importlib.util
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location('catalog_import', ROOT / 'scripts/catalog/import-opensource-l.py')
IMPORT = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(IMPORT)


class EndpointExtraction(unittest.TestCase):
    def extract(self, expression):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            adapter = root / 'litellm/llms/example/chat/transformation.py'
            adapter.parent.mkdir(parents=True)
            adapter.write_text('def _get_openai_compatible_provider_info(api_base, region):\n'
                               f'    api_base = api_base or {expression}\n'
                               '    return api_base, None\n')
            return IMPORT.adapter_endpoint(root, ['example'])

    def test_regional_url_fragments_do_not_become_fixed_endpoints(self):
        self.assertEqual(self.extract('f"https://gateway.{region}.example/v1"'), {})

    def test_complete_static_default_is_retained(self):
        result = self.extract('"https://gateway.example/v1"')
        self.assertEqual(result['base_url'], 'https://gateway.example/v1')
        self.assertIn('source_location', result)


if __name__ == '__main__':
    unittest.main()
