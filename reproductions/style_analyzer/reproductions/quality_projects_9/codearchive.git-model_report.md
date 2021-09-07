# Model report for file:///tmp/top-repos-quality-repos-vchuedkx/codearchive.git HEAD 34e5f13f89f383a68790da61ff2f2e3c1094cd45

### Dump

```json
{'created_at': '2021-08-31 22:19:16',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '19.1 kB',
 'tags': [],
 'uuid': '42eac01c-05fd-4a9e-9ed3-cd936b89e813',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-vchuedkx/codearchive.git 34e5f13f89f383a68790da61ff2f2e3c1094cd45

# javascript
28 rules, avg.len. 8.2
## train
PPCR: 0.869490
### report
macro
{'f1-score': 0.45482570929550603,
 'precision': 0.452611191611738,
 'recall': 0.4578644440056423,
 'support': 33238}
micro
{'f1-score': 0.9479511402611469,
 'precision': 0.9479511402611469,
 'recall': 0.9479511402611469,
 'support': 33238}
weighted
{'f1-score': 0.9372219345118403,
 'precision': 0.9268322689708953,
 'recall': 0.9479511402611469,
 'support': 33238}
### report_full
macro
{'f1-score': 0.39448853526167654,
 'precision': 0.452611191611738,
 'recall': 0.3611194591387926,
 'support': 38227}
micro
{'f1-score': 0.8817742951094941,
 'precision': 0.9479511402611469,
 'recall': 0.8242341800298219,
 'support': 38227}
weighted
{'f1-score': 0.8495272946878806,
 'precision': 0.8828178758856509,
 'recall': 0.8242341800298219,
 'support': 38227}
## test
PPCR: 0.878237
### report
macro
{'f1-score': 0.46885562798310526,
 'precision': 0.4651376261081478,
 'recall': 0.47320255672057765,
 'support': 8208}
micro
{'f1-score': 0.9578460038986355,
 'precision': 0.9578460038986355,
 'recall': 0.9578460038986355,
 'support': 8208}
weighted
{'f1-score': 0.9487610348433354,
 'precision': 0.9399917597870721,
 'recall': 0.9578460038986355,
 'support': 8208}
### report_full
macro
{'f1-score': 0.39603322793030804,
 'precision': 0.4651376261081478,
 'recall': 0.3603006055988188,
 'support': 9346}
micro
{'f1-score': 0.8957502563518287,
 'precision': 0.9578460038986355,
 'recall': 0.8412154932591484,
 'support': 9346}
weighted
{'f1-score': 0.8669417865339866,
 'precision': 0.9010111135556831,
 'recall': 0.8412154932591484,
 'support': 9346}
```

## javascript
### Summary
12 rules, avg.len. 7.2

| | |
|-|-|
|Min support|103|
|Max support|15712|
|Min confidence|0.923509955406189|
|Max confidence|0.9991227984428406|

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
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
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
| 1 | `  •••start_line ≥ 45<br>	∧ -1.internal_type = StringLiteral<br>	∧ -5.label in {<space>}<br>⇒ y = "<br>Confidence: 0.998. Support: 229.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ -5.label not in {<space>}<br>⇒ y = "<br>Confidence: 0.995. Support: 1155.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.999. Support: 570.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.965. Support: 216.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.924. Support: 1510.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 263.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 206.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, {}<br>	∧ -2.roles not in {MAP}<br>	∧ -4.diff_line ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {,, >}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 103.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = import<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.997. Support: 147.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {ClassProperty, Program}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 285.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.diff_col ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {/}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ +3.reserved not in {<}<br>	∧ ^1.internal_type not in {ClassProperty, Program}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 800.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, ;, }}<br>	∧ -1.roles not in {COMMENT}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {MAP}<br>	∧ -5.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {ClassProperty, ConditionalExpression, Program}<br>	∧ ^1.roles not in {BLOCK, DECLARATION, TYPE}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 15712.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.25, "max_conf": 0.9991227984428406, "max_support": 15712, "min_conf": 0.923509955406189, "min_support": 103, "num_rules": 12}}
```
</details>
