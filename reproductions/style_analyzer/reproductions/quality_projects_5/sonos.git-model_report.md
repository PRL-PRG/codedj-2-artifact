# Model report for file:///tmp/top-repos-quality-repos-in1nv9w1/sonos.git HEAD 787364aa6e1e9bdcc7828f71a3ffca5da680c3e7

### Dump

```json
{'created_at': '2021-09-02 11:12:14',
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
 'size': '16.9 kB',
 'tags': [],
 'uuid': '9049a0d1-b155-41e5-8dd5-ac9f668dd8e1',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-in1nv9w1/sonos.git 787364aa6e1e9bdcc7828f71a3ffca5da680c3e7

# javascript
14 rules, avg.len. 5.8
## train
PPCR: 0.792737
### report
macro
{'f1-score': 0.6642310516843091,
 'precision': 0.7000219108035866,
 'recall': 0.6381099698549504,
 'support': 13207}
micro
{'f1-score': 0.9184523358824865,
 'precision': 0.9184523358824865,
 'recall': 0.9184523358824865,
 'support': 13207}
weighted
{'f1-score': 0.9087479933455653,
 'precision': 0.9049048388480168,
 'recall': 0.9184523358824865,
 'support': 13207}
### report_full
macro
{'f1-score': 0.5556991702978882,
 'precision': 0.7000219108035866,
 'recall': 0.4884222113396606,
 'support': 16660}
micro
{'f1-score': 0.8122677202263369,
 'precision': 0.9184523358824865,
 'recall': 0.7280912364945978,
 'support': 16660}
weighted
{'f1-score': 0.7812083005994584,
 'precision': 0.8991300819939178,
 'recall': 0.7280912364945978,
 'support': 16660}
## test
PPCR: 0.756532
### report
macro
{'f1-score': 0.6768892312432231,
 'precision': 0.7062641618677508,
 'recall': 0.6538654422772152,
 'support': 3272}
micro
{'f1-score': 0.9303178484107579,
 'precision': 0.9303178484107579,
 'recall': 0.9303178484107579,
 'support': 3272}
weighted
{'f1-score': 0.9253253257452505,
 'precision': 0.9235161969086508,
 'recall': 0.9303178484107579,
 'support': 3272}
### report_full
macro
{'f1-score': 0.567918848838775,
 'precision': 0.7062641618677508,
 'recall': 0.4997217298240736,
 'support': 4325}
micro
{'f1-score': 0.8013689614321442,
 'precision': 0.9303178484107579,
 'recall': 0.7038150289017341,
 'support': 4325}
weighted
{'f1-score': 0.7694835329565013,
 'precision': 0.9006680647554881,
 'recall': 0.7038150289017341,
 'support': 4325}
```

## javascript
### Summary
8 rules, avg.len. 3.9

| | |
|-|-|
|Min support|103|
|Max support|963|
|Min confidence|0.9271844625473022|
|Max confidence|0.9985119104385376|

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
                     'min_samples_leaf': 101,
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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.936. Support: 963.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.939. Support: 190.` |
| 3 | `  -1.internal_type = CommentLine<br>⇒ y = ⏎<br>Confidence: 0.960. Support: 392.` |
| 4 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.996. Support: 132.` |
| 5 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved = )<br>⇒ y = ␣<br>Confidence: 0.927. Support: 103.` |
| 6 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.997. Support: 186.` |
| 7 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 336.` |
| 8 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 139.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.875, "max_conf": 0.9985119104385376, "max_support": 963, "min_conf": 0.9271844625473022, "min_support": 103, "num_rules": 8}}
```
</details>
