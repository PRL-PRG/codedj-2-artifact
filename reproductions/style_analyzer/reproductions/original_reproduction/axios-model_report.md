# Model report for file:///tmp/top-repos-quality-repos-q45bejf2/axios HEAD 21ae22dbd3ae3d3a55d9efd4eead3dd7fb6d8e6e

### Dump

```json
{'created_at': '2021-08-13 16:07:30',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-80-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.9 (default, Jan 26 2021, 15:33:00) [GCC 8.4.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '17.0 kB',
 'tags': [],
 'uuid': '936558f4-0006-4c10-aa25-0ec076a9a0fc',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-q45bejf2/axios 21ae22dbd3ae3d3a55d9efd4eead3dd7fb6d8e6e

# javascript
17 rules, avg.len. 7.0
## train
PPCR: 0.876295
### report
macro
{'f1-score': 0.7956971142738336,
 'precision': 0.8137287232021627,
 'recall': 0.7832853572467755,
 'support': 21733}
micro
{'f1-score': 0.9580361661988681,
 'precision': 0.9580361661988681,
 'recall': 0.9580361661988681,
 'support': 21733}
weighted
{'f1-score': 0.9559772419807132,
 'precision': 0.9548677752390569,
 'recall': 0.9580361661988681,
 'support': 21733}
### report_full
macro
{'f1-score': 0.6528913004106949,
 'precision': 0.8137287232021627,
 'recall': 0.6016601806369044,
 'support': 24801}
micro
{'f1-score': 0.8948725662956118,
 'precision': 0.9580361661988681,
 'recall': 0.8395225998951655,
 'support': 24801}
weighted
{'f1-score': 0.8778815863536196,
 'precision': 0.9509398886376859,
 'recall': 0.8395225998951655,
 'support': 24801}
## test
PPCR: 0.871912
### report
macro
{'f1-score': 0.7928211575608576,
 'precision': 0.856498649138005,
 'recall': 0.7587430791623229,
 'support': 4976}
micro
{'f1-score': 0.9662379421221865,
 'precision': 0.9662379421221865,
 'recall': 0.9662379421221865,
 'support': 4976}
weighted
{'f1-score': 0.9652497139554062,
 'precision': 0.9668587527493724,
 'recall': 0.9662379421221865,
 'support': 4976}
### report_full
macro
{'f1-score': 0.6566858557723882,
 'precision': 0.856498649138005,
 'recall': 0.6013236228010279,
 'support': 5707}
micro
{'f1-score': 0.9001216886642329,
 'precision': 0.9662379421221865,
 'recall': 0.8424741545470474,
 'support': 5707}
weighted
{'f1-score': 0.8821099850041337,
 'precision': 0.9686680016984801,
 'recall': 0.8424741545470474,
 'support': 5707}
```

## javascript
### Summary
17 rules, avg.len. 7.0

| | |
|-|-|
|Min support|134|
|Max support|7847|
|Min confidence|0.8146852850914001|
|Max confidence|0.9987775087356567|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 130,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 130,
                     'min_samples_split': 240,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.roles in {STRING}<br>⇒ y = '<br>Confidence: 0.968. Support: 1053.` |
| 2 | `  -1.reserved not in {,}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles in {STRING}<br>⇒ y = '<br>Confidence: 0.849. Support: 1172.` |
| 3 | `  -1.reserved = ;<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {STRING}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.990. Support: 541.` |
| 4 | `  -1.reserved = ;<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {STRING}<br>	∧ +3.internal_type = StringLiteral<br>⇒ y = ⏎⏎<br>Confidence: 0.966. Support: 162.` |
| 5 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.903. Support: 811.` |
| 6 | `  -1.reserved not in {;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {STRING}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 840.` |
| 7 | `  -1.reserved not in {;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.881. Support: 883.` |
| 8 | `  -1.reserved not in {;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 496.` |
| 9 | `  -1.reserved = ,<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.815. Support: 143.` |
| 10 | `  -1.reserved = ,<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 465.` |
| 11 | `  -1.reserved = function<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.903. Support: 440.` |
| 12 | `  -1.reserved = var<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 308.` |
| 13 | `  -1.reserved = :<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 214.` |
| 14 | `  -1.reserved not in {,, :, ;, function, var, {}<br>	∧ -1.roles in {EXPRESSION} and not in {COMMENT, STRING}<br>	∧ -2.label in {<newline>}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {FILE, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 1227.` |
| 15 | `  -1.diff_offset ≥ 2<br>	∧ -1.reserved not in {;, function, var, {}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.label in {<newline>}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {FILE, IF}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 134.` |
| 16 | `  -1.diff_offset ≤ 1<br>	∧ -1.reserved not in {,, :, ;, function, var, {}<br>	∧ -1.roles not in {COMMENT, EXPRESSION, STRING}<br>	∧ -2.label in {<newline>}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {FILE, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.910. Support: 630.` |
| 17 | `  -1.reserved not in {,, :, ;, function, var, {}<br>	∧ -1.roles not in {COMMENT, STRING}<br>	∧ -2.label not in {<newline>}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {FILE, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 7847.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.0, "max_conf": 0.9987775087356567, "max_support": 7847, "min_conf": 0.8146852850914001, "min_support": 134, "num_rules": 17}}
```
</details>
