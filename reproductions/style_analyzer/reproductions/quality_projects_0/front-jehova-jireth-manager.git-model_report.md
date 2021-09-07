# Model report for file:///tmp/top-repos-quality-repos-36b2r0a0/front-jehova-jireth-manager.git HEAD ce0b7044d85f1608ca888d72f8b69856f0b4df0a

### Dump

```json
{'created_at': '2021-09-01 17:58:27',
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
 'size': '16.3 kB',
 'tags': [],
 'uuid': '98ae1997-3e0c-48d0-a6e7-3a1a9f61ecde',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-36b2r0a0/front-jehova-jireth-manager.git ce0b7044d85f1608ca888d72f8b69856f0b4df0a

# javascript
11 rules, avg.len. 5.1
## train
PPCR: 0.721850
### report
macro
{'f1-score': 0.16829012032153198,
 'precision': 0.20409572235490106,
 'recall': 0.1627775443340154,
 'support': 3231}
micro
{'f1-score': 0.8108944599195297,
 'precision': 0.8108944599195296,
 'recall': 0.8108944599195296,
 'support': 3231}
weighted
{'f1-score': 0.7595500461784642,
 'precision': 0.7582505306866303,
 'recall': 0.8108944599195296,
 'support': 3231}
### report_full
macro
{'f1-score': 0.1445228386714277,
 'precision': 0.20409572235490106,
 'recall': 0.13496544110687228,
 'support': 4476}
micro
{'f1-score': 0.6799013883482548,
 'precision': 0.8108944599195296,
 'recall': 0.5853440571939231,
 'support': 4476}
weighted
{'f1-score': 0.582821736467575,
 'precision': 0.6607299723346738,
 'recall': 0.5853440571939231,
 'support': 4476}
## test
PPCR: 0.822723
### report
macro
{'f1-score': 0.1353470380194518,
 'precision': 0.19688644688644688,
 'recall': 0.13994834771379938,
 'support': 840}
micro
{'f1-score': 0.7666666666666667,
 'precision': 0.7666666666666667,
 'recall': 0.7666666666666667,
 'support': 840}
weighted
{'f1-score': 0.6846777188328911,
 'precision': 0.7051892551892552,
 'recall': 0.7666666666666667,
 'support': 840}
### report_full
macro
{'f1-score': 0.12480671063417542,
 'precision': 0.19688644688644688,
 'recall': 0.1275269001831502,
 'support': 1021}
micro
{'f1-score': 0.692101020956475,
 'precision': 0.7666666666666667,
 'recall': 0.6307541625857003,
 'support': 1021}
weighted
{'f1-score': 0.583621491710082,
 'precision': 0.6584903832700111,
 'recall': 0.6307541625857003,
 'support': 1021}
```

## javascript
### Summary
1 rules, avg.len. 7.0

| | |
|-|-|
|Min support|160|
|Max support|160|
|Min confidence|0.965624988079071|
|Max confidence|0.965624988079071|

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
                     'min_samples_leaf': 90,
                     'min_samples_split': 182,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {Program}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 160.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.0, "max_conf": 0.965624988079071, "max_support": 160, "min_conf": 0.965624988079071, "min_support": 160, "num_rules": 1}}
```
</details>
