# Model report for file:///tmp/top-repos-quality-repos-qto1_5n7/hotel.git HEAD cf1c5882ea3bd744fe78e974791534a27056ed7c

### Dump

```json
{'created_at': '2021-09-01 20:10:11',
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
 'size': '16.9 kB',
 'tags': [],
 'uuid': '9b69a67c-b372-4a70-b6a9-7986309eac67',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-qto1_5n7/hotel.git cf1c5882ea3bd744fe78e974791534a27056ed7c

# javascript
14 rules, avg.len. 6.1
## train
PPCR: 0.907838
### report
macro
{'f1-score': 0.8092642452625685,
 'precision': 0.824195271893494,
 'recall': 0.7982707354646379,
 'support': 4807}
micro
{'f1-score': 0.9407114624505928,
 'precision': 0.9407114624505929,
 'recall': 0.9407114624505929,
 'support': 4807}
weighted
{'f1-score': 0.9352338454337767,
 'precision': 0.9325313154049604,
 'recall': 0.9407114624505929,
 'support': 4807}
### report_full
macro
{'f1-score': 0.7824560507788179,
 'precision': 0.824195271893494,
 'recall': 0.751870852222985,
 'support': 5295}
micro
{'f1-score': 0.8952682637101563,
 'precision': 0.9407114624505929,
 'recall': 0.8540132200188857,
 'support': 5295}
weighted
{'f1-score': 0.8865954153644268,
 'precision': 0.928052011911766,
 'recall': 0.8540132200188857,
 'support': 5295}
## test
PPCR: 0.875889
### report
macro
{'f1-score': 0.5836650352275352,
 'precision': 0.5879676530843051,
 'recall': 0.6037844046417029,
 'support': 1108}
micro
{'f1-score': 0.907942238267148,
 'precision': 0.907942238267148,
 'recall': 0.907942238267148,
 'support': 1108}
weighted
{'f1-score': 0.8943214675728215,
 'precision': 0.8886832006177972,
 'recall': 0.907942238267148,
 'support': 1108}
### report_full
macro
{'f1-score': 0.5498307889464641,
 'precision': 0.5879676530843051,
 'recall': 0.5581595522332669,
 'support': 1265}
micro
{'f1-score': 0.8478718921196797,
 'precision': 0.907942238267148,
 'recall': 0.7952569169960474,
 'support': 1265}
weighted
{'f1-score': 0.8224226781815291,
 'precision': 0.8763966394847039,
 'recall': 0.7952569169960474,
 'support': 1265}
```

## javascript
### Summary
10 rules, avg.len. 5.8

| | |
|-|-|
|Min support|96|
|Max support|1570|
|Min confidence|0.9246031641960144|
|Max confidence|0.9981273412704468|

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
| 1 | `  -1.reserved = ,<br>	∧ ^1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.984. Support: 279.` |
| 2 | `  -1.reserved = ,<br>	∧ ^1.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 100.` |
| 3 | `  -1.reserved = :<br>⇒ y = ␣<br>Confidence: 0.998. Support: 267.` |
| 4 | `  -1.reserved = {<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.990. Support: 151.` |
| 5 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,, :, {}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, MAP}<br>⇒ y = '<br>Confidence: 0.939. Support: 122.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ +5.roles in {MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = "<br>Confidence: 0.964. Support: 96.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ +5.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.926. Support: 101.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {OPERATOR} and not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 126.` |
| 9 | `  -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 3<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 121.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 2<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 1570.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.8, "max_conf": 0.9981273412704468, "max_support": 1570, "min_conf": 0.9246031641960144, "min_support": 96, "num_rules": 10}}
```
</details>
