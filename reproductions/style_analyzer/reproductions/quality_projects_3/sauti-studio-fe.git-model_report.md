# Model report for file:///tmp/top-repos-quality-repos-g4pj7g7b/sauti-studio-fe.git HEAD e3905ede28b7fa2e41488547d2ddad6d115337b6

### Dump

```json
{'created_at': '2021-09-02 07:18:34',
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
 'uuid': 'e2110c86-8b6d-4db6-871a-e2c94e722bd2',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-g4pj7g7b/sauti-studio-fe.git e3905ede28b7fa2e41488547d2ddad6d115337b6

# javascript
6 rules, avg.len. 4.5
## train
PPCR: 0.569988
### report
macro
{'f1-score': 0.3104529293278586,
 'precision': 0.34224144329657197,
 'recall': 0.29331164686880423,
 'support': 4919}
micro
{'f1-score': 0.9288473266924172,
 'precision': 0.9288473266924172,
 'recall': 0.9288473266924172,
 'support': 4919}
weighted
{'f1-score': 0.9114701997640027,
 'precision': 0.9042261468249387,
 'recall': 0.9288473266924172,
 'support': 4919}
### report_full
macro
{'f1-score': 0.1937997754344299,
 'precision': 0.34224144329657197,
 'recall': 0.15983653505124207,
 'support': 8630}
micro
{'f1-score': 0.6744409181489408,
 'precision': 0.9288473266924172,
 'recall': 0.5294322132097334,
 'support': 8630}
weighted
{'f1-score': 0.57552935453185,
 'precision': 0.7732199247226763,
 'recall': 0.5294322132097334,
 'support': 8630}
## test
PPCR: 0.511914
### report
macro
{'f1-score': 0.2200689071566731,
 'precision': 0.23995732347060647,
 'recall': 0.27483199594630514,
 'support': 666}
micro
{'f1-score': 0.8978978978978979,
 'precision': 0.8978978978978979,
 'recall': 0.8978978978978979,
 'support': 666}
weighted
{'f1-score': 0.8829549230081145,
 'precision': 0.8914497897767115,
 'recall': 0.8978978978978979,
 'support': 666}
### report_full
macro
{'f1-score': 0.13980905258116677,
 'precision': 0.23995732347060647,
 'recall': 0.11807089878896211,
 'support': 1301}
micro
{'f1-score': 0.6080325368581596,
 'precision': 0.8978978978978979,
 'recall': 0.45964642582628745,
 'support': 1301}
weighted
{'f1-score': 0.5234578221279215,
 'precision': 0.727601100831234,
 'recall': 0.45964642582628745,
 'support': 1301}
```

## javascript
### Summary
3 rules, avg.len. 4.7

| | |
|-|-|
|Min support|113|
|Max support|513|
|Min confidence|0.9247787594795227|
|Max confidence|0.9990253448486328|

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
                     'min_samples_split': 239,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  +1.length ≥ 2<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 513.` |
| 2 | `  +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 247.` |
| 3 | `  -1.reserved not in {(, ;, {}<br>	∧ -4.length ≥ 3<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.925. Support: 113.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.666666666666667, "max_conf": 0.9990253448486328, "max_support": 513, "min_conf": 0.9247787594795227, "min_support": 113, "num_rules": 3}}
```
</details>
