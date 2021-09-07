# Model report for file:///tmp/top-repos-quality-repos-110j7_xc/metrics-module.git HEAD 08be393709d9bca4221dc9d05653cfd1b5935a7b

### Dump

```json
{'created_at': '2021-09-02 03:10:22',
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
 'size': '15.9 kB',
 'tags': [],
 'uuid': 'd5af5346-4349-4607-80e1-ea494bbc7361',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-110j7_xc/metrics-module.git 08be393709d9bca4221dc9d05653cfd1b5935a7b

# javascript
12 rules, avg.len. 4.6
## train
PPCR: 0.816357
### report
macro
{'f1-score': 0.6621954822003,
 'precision': 0.6640239997510499,
 'recall': 0.6609891020173322,
 'support': 4961}
micro
{'f1-score': 0.9381173150574481,
 'precision': 0.9381173150574481,
 'recall': 0.9381173150574481,
 'support': 4961}
weighted
{'f1-score': 0.9305888752633027,
 'precision': 0.9235001740162793,
 'recall': 0.9381173150574481,
 'support': 4961}
### report_full
macro
{'f1-score': 0.6247142731601864,
 'precision': 0.6640239997510499,
 'recall': 0.5920824263061375,
 'support': 6077}
micro
{'f1-score': 0.8432687080992933,
 'precision': 0.9381173150574481,
 'recall': 0.7658384071087708,
 'support': 6077}
weighted
{'f1-score': 0.8076627731549347,
 'precision': 0.8561339819156574,
 'recall': 0.7658384071087708,
 'support': 6077}
## test
PPCR: 0.834966
### report
macro
{'f1-score': 0.651299869170512,
 'precision': 0.6708349495060248,
 'recall': 0.6351689001780969,
 'support': 1108}
micro
{'f1-score': 0.9494584837545126,
 'precision': 0.9494584837545126,
 'recall': 0.9494584837545126,
 'support': 1108}
weighted
{'f1-score': 0.9431319365069561,
 'precision': 0.938465357674884,
 'recall': 0.9494584837545126,
 'support': 1108}
### report_full
macro
{'f1-score': 0.6146592570815511,
 'precision': 0.6708349495060248,
 'recall': 0.5689986487362203,
 'support': 1327}
micro
{'f1-score': 0.8640657084188912,
 'precision': 0.9494584837545126,
 'recall': 0.7927656367746797,
 'support': 1327}
weighted
{'f1-score': 0.8322386551877659,
 'precision': 0.878152881476235,
 'recall': 0.7927656367746797,
 'support': 1327}
```

## javascript
### Summary
8 rules, avg.len. 3.8

| | |
|-|-|
|Min support|119|
|Max support|894|
|Min confidence|0.9361110925674438|
|Max confidence|0.9973404407501221|

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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.980. Support: 810.` |
| 2 | `  -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.987. Support: 119.` |
| 3 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.943. Support: 237.` |
| 4 | `  -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.936. Support: 180.` |
| 5 | `  -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.997. Support: 180.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 188.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.937. Support: 183.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 894.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.75, "max_conf": 0.9973404407501221, "max_support": 894, "min_conf": 0.9361110925674438, "min_support": 119, "num_rules": 8}}
```
</details>
