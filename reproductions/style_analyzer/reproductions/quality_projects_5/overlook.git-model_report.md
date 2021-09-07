# Model report for file:///tmp/top-repos-quality-repos-gjj7uwer/overlook.git HEAD 3304b501b9fe71244891a33c692c735eec4b7f89

### Dump

```json
{'created_at': '2021-09-02 10:40:24',
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
 'size': '16.8 kB',
 'tags': [],
 'uuid': '39bc7ab8-70e6-4d8b-ba0c-da35d63c5200',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-gjj7uwer/overlook.git 3304b501b9fe71244891a33c692c735eec4b7f89

# javascript
10 rules, avg.len. 5.2
## train
PPCR: 0.751571
### report
macro
{'f1-score': 0.6730962943635422,
 'precision': 0.6911699223255242,
 'recall': 0.6601520802826935,
 'support': 5261}
micro
{'f1-score': 0.9218779699676869,
 'precision': 0.9218779699676868,
 'recall': 0.9218779699676868,
 'support': 5261}
weighted
{'f1-score': 0.9129098919355663,
 'precision': 0.906703749564202,
 'recall': 0.9218779699676868,
 'support': 5261}
### report_full
macro
{'f1-score': 0.5170133054266921,
 'precision': 0.6911699223255242,
 'recall': 0.436536937901621,
 'support': 7000}
micro
{'f1-score': 0.7911263355354377,
 'precision': 0.9218779699676868,
 'recall': 0.6928571428571428,
 'support': 7000}
weighted
{'f1-score': 0.7610939441178487,
 'precision': 0.8913599267344645,
 'recall': 0.6928571428571428,
 'support': 7000}
## test
PPCR: 0.649789
### report
macro
{'f1-score': 0.6860995098867647,
 'precision': 0.6865420505674793,
 'recall': 0.6890613615389669,
 'support': 770}
micro
{'f1-score': 0.925974025974026,
 'precision': 0.925974025974026,
 'recall': 0.925974025974026,
 'support': 770}
weighted
{'f1-score': 0.9163681265461181,
 'precision': 0.9098637980508915,
 'recall': 0.925974025974026,
 'support': 770}
### report_full
macro
{'f1-score': 0.5180072966711952,
 'precision': 0.6865420505674793,
 'recall': 0.434865243218915,
 'support': 1185}
micro
{'f1-score': 0.7294117647058824,
 'precision': 0.925974025974026,
 'recall': 0.6016877637130802,
 'support': 1185}
weighted
{'f1-score': 0.7009852601892822,
 'precision': 0.9103324819351247,
 'recall': 0.6016877637130802,
 'support': 1185}
```

## javascript
### Summary
7 rules, avg.len. 4.6

| | |
|-|-|
|Min support|100|
|Max support|1127|
|Min confidence|0.925595223903656|
|Max confidence|0.9963503479957581|

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
                     'min_samples_split': 227,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -3.roles not in {CALLEE}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.954. Support: 1127.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ +2.roles in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = "<br>Confidence: 0.981. Support: 241.` |
| 3 | `  -1.internal_type = StringLiteral<br>	∧ +2.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.926. Support: 168.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 137.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 326.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.928. Support: 118.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 100.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.571428571428571, "max_conf": 0.9963503479957581, "max_support": 1127, "min_conf": 0.925595223903656, "min_support": 100, "num_rules": 7}}
```
</details>
