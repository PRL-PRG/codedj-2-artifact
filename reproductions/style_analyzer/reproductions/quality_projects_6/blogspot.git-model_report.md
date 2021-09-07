# Model report for file:///tmp/top-repos-quality-repos-jjdfw5jt/blogspot.git HEAD 8b0bf09fb493cf5af051f1c061f38ede9dee187c

### Dump

```json
{'created_at': '2021-09-01 23:44:21',
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
 'size': '19.4 kB',
 'tags': [],
 'uuid': '2d4be32a-6c39-45f1-8408-7c43433f972d',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-jjdfw5jt/blogspot.git 8b0bf09fb493cf5af051f1c061f38ede9dee187c

# javascript
12 rules, avg.len. 6.8
## train
PPCR: 0.795119
### report
macro
{'f1-score': 0.2325822440654477,
 'precision': 0.22684646897309224,
 'recall': 0.23880750942891701,
 'support': 13913}
micro
{'f1-score': 0.9358872996478114,
 'precision': 0.9358872996478114,
 'recall': 0.9358872996478114,
 'support': 13913}
weighted
{'f1-score': 0.9253573998903797,
 'precision': 0.9155222679613205,
 'recall': 0.9358872996478114,
 'support': 13913}
### report_full
macro
{'f1-score': 0.19904904685913358,
 'precision': 0.22684646897309224,
 'recall': 0.1864490973703392,
 'support': 17498}
micro
{'f1-score': 0.8290726178727198,
 'precision': 0.9358872996478114,
 'recall': 0.7441421876785919,
 'support': 17498}
weighted
{'f1-score': 0.7552655153053243,
 'precision': 0.7763387269680772,
 'recall': 0.7441421876785919,
 'support': 17498}
## test
PPCR: 0.804179
### report
macro
{'f1-score': 0.23601365010608602,
 'precision': 0.2319586412145004,
 'recall': 0.24157231374103919,
 'support': 3733}
micro
{'f1-score': 0.9399946423787838,
 'precision': 0.9399946423787838,
 'recall': 0.9399946423787838,
 'support': 3733}
weighted
{'f1-score': 0.9314885487997161,
 'precision': 0.9268553807163362,
 'recall': 0.9399946423787838,
 'support': 3733}
### report_full
macro
{'f1-score': 0.20640629041919353,
 'precision': 0.2319586412145004,
 'recall': 0.19755543139498896,
 'support': 4642}
micro
{'f1-score': 0.8379701492537314,
 'precision': 0.9399946423787838,
 'recall': 0.7559241706161137,
 'support': 4642}
weighted
{'f1-score': 0.7696668446786561,
 'precision': 0.7976091330183845,
 'recall': 0.7559241706161137,
 'support': 4642}
```

## javascript
### Summary
8 rules, avg.len. 5.9

| | |
|-|-|
|Min support|108|
|Max support|3164|
|Min confidence|0.9295227527618408|
|Max confidence|0.9972677826881409|

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
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 3164.` |
| 2 | `  -1.internal_type = Identifier<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 183.` |
| 3 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 1792.` |
| 4 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 901.` |
| 5 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 639.` |
| 6 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 253.` |
| 7 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.reserved not in {), ,, ;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 117.` |
| 8 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.reserved not in {), ,, ;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.949. Support: 108.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.875, "max_conf": 0.9972677826881409, "max_support": 3164, "min_conf": 0.9295227527618408, "min_support": 108, "num_rules": 8}}
```
</details>
