# Model report for file:///tmp/top-repos-quality-repos-13uctz3q/cffbrkwb.git HEAD 8db8e4492380f6c8875835b83539262438606432

### Dump

```json
{'created_at': '2021-09-01 17:06:26',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-74-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '15.8 kB',
 'tags': [],
 'uuid': '8d0c5a35-9bc2-4879-8690-851227b41a9f',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-13uctz3q/cffbrkwb.git 8db8e4492380f6c8875835b83539262438606432

# javascript
57 rules, avg.len. 4.7
## train
PPCR: 0.989086
### report
macro
{'f1-score': 0.5168881816890006,
 'precision': 0.5780869291180952,
 'recall': 0.49015707555267873,
 'support': 8700}
micro
{'f1-score': 0.8480459770114942,
 'precision': 0.8480459770114942,
 'recall': 0.8480459770114942,
 'support': 8700}
weighted
{'f1-score': 0.8265144149422451,
 'precision': 0.8298651709229613,
 'recall': 0.8480459770114942,
 'support': 8700}
### report_full
macro
{'f1-score': 0.5136227237262493,
 'precision': 0.5780869291180952,
 'recall': 0.48420233931835827,
 'support': 8796}
micro
{'f1-score': 0.8433927754915408,
 'precision': 0.8480459770114942,
 'recall': 0.8387903592542064,
 'support': 8796}
weighted
{'f1-score': 0.819840043769974,
 'precision': 0.8257893458652921,
 'recall': 0.8387903592542064,
 'support': 8796}
## test
PPCR: 0.995643
### report
macro
{'f1-score': 0.40961657401157553,
 'precision': 0.4399132961144309,
 'recall': 0.40073346515087144,
 'support': 1828}
micro
{'f1-score': 0.800875273522976,
 'precision': 0.8008752735229759,
 'recall': 0.8008752735229759,
 'support': 1828}
weighted
{'f1-score': 0.7739351656456254,
 'precision': 0.7793276856920267,
 'recall': 0.8008752735229759,
 'support': 1828}
### report_full
macro
{'f1-score': 0.40928045085605125,
 'precision': 0.4399132961144309,
 'recall': 0.4000628043264058,
 'support': 1836}
micro
{'f1-score': 0.7991266375545851,
 'precision': 0.8008752735229759,
 'recall': 0.7973856209150327,
 'support': 1836}
weighted
{'f1-score': 0.7707946703241267,
 'precision': 0.7763965631883143,
 'recall': 0.7973856209150327,
 'support': 1836}
```

## javascript
### Summary
26 rules, avg.len. 4.4

| | |
|-|-|
|Min support|140|
|Max support|2459|
|Min confidence|0.921875|
|Max confidence|0.9986910820007324|

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
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
                     'confidence_threshold': 0.8,
                     'max_features': 'auto',
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  +1.roles in {MAP}<br>	∧ +3.roles in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.963. Support: 147.` |
| 2 | `  -4.roles in {INCOMPLETE}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +3.roles not in {LITERAL}<br>⇒ y = "<br>Confidence: 0.998. Support: 275.` |
| 3 | `  -1.reserved = :<br>	∧ -1.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.roles not in {LITERAL}<br>	∧ ^1.roles not in {MODULE}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 150.` |
| 4 | `  •••start_col ≥ 28<br>	∧ -2.diff_offset ≥ 9<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ +5.roles not in {KEY}<br>⇒ y = "<br>Confidence: 0.988. Support: 204.` |
| 5 | `  •••start_col ≤ 27<br>	∧ +5.roles not in {KEY}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.997. Support: 1598.` |
| 6 | `  •••start_col ≤ 27<br>	∧ -3.reserved = ,<br>	∧ +5.roles not in {KEY}<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 165.` |
| 7 | `  •••start_col ≤ 27<br>	∧ -3.internal_type = JSXText<br>	∧ -3.reserved not in {,}<br>	∧ +5.roles not in {KEY}<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 294.` |
| 8 | `  -2.roles in {STRING}<br>	∧ -4.reserved not in {:}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 160.` |
| 9 | `  -2.reserved not in {=}<br>	∧ -2.roles not in {STRING}<br>	∧ -3.diff_line = 0<br>	∧ -3.label not in {<space>}<br>	∧ -4.reserved not in {:}<br>	∧ +3.roles not in {LITERAL}<br>	∧ ^2.internal_type = Program<br>⇒ y = ␣<br>Confidence: 0.961. Support: 243.` |
| 10 | `  -2.reserved not in {<}<br>	∧ -5.diff_line ≥ 1<br>	∧ +3.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 1323.` |
| 11 | `  -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {<}<br>	∧ -5.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {>}<br>	∧ +3.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.935. Support: 1550.` |
| 12 | `  -1.roles in {KEY}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 235.` |
| 13 | `  -1.roles not in {KEY, STRING}<br>	∧ +4.reserved = ,<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ⏎<br>Confidence: 0.932. Support: 155.` |
| 14 | `  •••start_line ≥ 48<br>	∧ -1.roles in {EXPRESSION} and not in {KEY, STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +4.reserved not in {,}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.931. Support: 255.` |
| 15 | `  -1.roles in {EXPRESSION, LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.roles in {UNANNOTATED}<br>⇒ y = "<br>Confidence: 0.997. Support: 181.` |
| 16 | `  -1.diff_offset ≥ 3<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +5.roles in {INCOMPLETE}<br>	∧ ^2.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 271.` |
| 17 | `  -1.diff_offset ≥ 3<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = =<br>	∧ +5.roles not in {INCOMPLETE}<br>	∧ ^2.roles not in {FILE}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 273.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = :<br>	∧ +3.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.965. Support: 186.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.roles not in {IMPORT}<br>	∧ +3.roles not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 1525.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.diff_line = 0<br>	∧ +1.internal_type = JSXIdentifier<br>	∧ +1.roles not in {IMPORT, STRING}<br>	∧ +3.roles in {EXPRESSION} and not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 382.` |
| 21 | `  -2.roles in {VALUE}<br>⇒ y = ⏎<br>Confidence: 0.982. Support: 140.` |
| 22 | `  -1.reserved = :<br>	∧ -2.roles not in {VALUE}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 198.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles in {INCOMPLETE}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +3.internal_type not in {StringLiteral}<br>⇒ y = "<br>Confidence: 0.999. Support: 361.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION, INITIALIZATION}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 161.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION, INITIALIZATION}<br>	∧ ^2.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.962. Support: 2459.` |
| 26 | `  -2.reserved = <<br>	∧ +3.roles in {STRING}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 250.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.384615384615385, "max_conf": 0.9986910820007324, "max_support": 2459, "min_conf": 0.921875, "min_support": 140, "num_rules": 26}}
```
</details>
