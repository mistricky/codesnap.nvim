from glob import glob
from typing import Callable, Final
from re import sub 
from functools import reduce
import tomllib

PROJECT_CONFIG_PATH: Final = "project.toml"
TEMPLATE_PATH: Final = "template/**/*.template"

read_config: Callable[[str], dict] = lambda path: tomllib.load(open(path, 'rb'))

apply_config: Callable[[dict, list[str]], str] = lambda config, keys: reduce(lambda config, key: config[key], keys, config)

replace: Callable[[str, dict], str] = lambda content, config: sub(r"{{([^}]+)}}", lambda match: apply_config(config, match.group(1).split('.')), content)

update: Callable[[str, str], None] = lambda path, content: open(sub(r'^template/(.+)\.template$', r"\1", path), 'w').write(content)

for template in map(lambda file: {'path': file, 'content': replace(open(file, 'r').read(), read_config(PROJECT_CONFIG_PATH))}, glob(TEMPLATE_PATH)):
    update(template['path'], template['content'])
