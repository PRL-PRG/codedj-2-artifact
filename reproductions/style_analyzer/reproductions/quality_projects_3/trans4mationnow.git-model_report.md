# Model report for file:///tmp/top-repos-quality-repos-aqrws9qf/trans4mationnow.git HEAD fa4ae9a3dd29627f02bcbfb8de842c31ce548e61

### Dump

```json
{'created_at': '2021-09-02 06:15:09',
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
 'size': '18.4 kB',
 'tags': [],
 'uuid': '65d1f37f-3578-4695-96e7-47aaa98ff5dc',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-aqrws9qf/trans4mationnow.git fa4ae9a3dd29627f02bcbfb8de842c31ce548e61

# javascript
14 rules, avg.len. 5.9
## train
PPCR: 0.828603
### report
macro
{'f1-score': 0.3323150420032137,
 'precision': 0.35378265748508786,
 'recall': 0.32267583389524457,
 'support': 13517}
micro
{'f1-score': 0.8891026115262263,
 'precision': 0.8891026115262263,
 'recall': 0.8891026115262263,
 'support': 13517}
weighted
{'f1-score': 0.8662835077335614,
 'precision': 0.8525030646282861,
 'recall': 0.8891026115262263,
 'support': 13517}
### report_full
macro
{'f1-score': 0.2745735596232624,
 'precision': 0.35378265748508786,
 'recall': 0.2491289584541656,
 'support': 16313}
micro
{'f1-score': 0.8057660073751257,
 'precision': 0.8891026115262263,
 'recall': 0.7367130509409673,
 'support': 16313}
weighted
{'f1-score': 0.7527498139074384,
 'precision': 0.8076595415143131,
 'recall': 0.7367130509409673,
 'support': 16313}
## test
PPCR: 0.814641
### report
macro
{'f1-score': 0.2766770793832665,
 'precision': 0.36700927407037265,
 'recall': 0.27552640471121215,
 'support': 3116}
micro
{'f1-score': 0.8889602053915276,
 'precision': 0.8889602053915276,
 'recall': 0.8889602053915276,
 'support': 3116}
weighted
{'f1-score': 0.859366476730093,
 'precision': 0.8540432686054794,
 'recall': 0.8889602053915276,
 'support': 3116}
### report_full
macro
{'f1-score': 0.22817946492997382,
 'precision': 0.36700927407037265,
 'recall': 0.20387762852572164,
 'support': 3825}
micro
{'f1-score': 0.7981558853191183,
 'precision': 0.8889602053915276,
 'recall': 0.7241830065359477,
 'support': 3825}
weighted
{'f1-score': 0.7405238664494261,
 'precision': 0.8371975335449341,
 'recall': 0.7241830065359477,
 'support': 3825}
```

## javascript
### Summary
7 rules, avg.len. 5.0

| | |
|-|-|
|Min support|90|
|Max support|405|
|Min confidence|0.925159215927124|
|Max confidence|0.9944444298744202|

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
                     'max_depth': 10,
                     'min_samples_leaf': 90,
                     'min_samples_split': 183,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles not in {INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.989. Support: 405.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.935. Support: 130.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.930. Support: 107.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 90.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -3.diff_col ≤ 4<br>	∧ -3.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {>}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 110.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.roles in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 314.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 93.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.0, "max_conf": 0.9944444298744202, "max_support": 405, "min_conf": 0.925159215927124, "min_support": 90, "num_rules": 7}}
```
</details>
