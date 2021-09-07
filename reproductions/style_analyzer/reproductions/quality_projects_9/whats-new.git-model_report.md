# Model report for file:///tmp/top-repos-quality-repos-rcqjsyj3/whats-new.git HEAD 199be4289d55e664cba2be26bf98297b70ce3df9

### Dump

```json
{'created_at': '2021-08-31 21:11:46',
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
 'size': '13.1 kB',
 'tags': [],
 'uuid': 'ad31a4ec-878f-4ab5-96b3-8e3236b7e1db',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-rcqjsyj3/whats-new.git 199be4289d55e664cba2be26bf98297b70ce3df9

# javascript
5 rules, avg.len. 2.8
## train
PPCR: 0.871408
### report
macro
{'f1-score': 0.895106517714039,
 'precision': 0.9304507629519839,
 'recall': 0.8883103057209725,
 'support': 1789}
micro
{'f1-score': 0.9100055897149245,
 'precision': 0.9100055897149245,
 'recall': 0.9100055897149245,
 'support': 1789}
weighted
{'f1-score': 0.90216001003022,
 'precision': 0.9199106255417255,
 'recall': 0.9100055897149245,
 'support': 1789}
### report_full
macro
{'f1-score': 0.8310282815316508,
 'precision': 0.9304507629519839,
 'recall': 0.7830809465821766,
 'support': 2053}
micro
{'f1-score': 0.8474752732951587,
 'precision': 0.9100055897149245,
 'recall': 0.7929858743302484,
 'support': 2053}
weighted
{'f1-score': 0.8331100089322482,
 'precision': 0.9208387919110758,
 'recall': 0.7929858743302484,
 'support': 2053}
## test
PPCR: 0.638743
### report
macro
{'f1-score': 0.491991991991992,
 'precision': 0.6677791262135923,
 'recall': 0.52,
 'support': 122}
micro
{'f1-score': 0.8032786885245902,
 'precision': 0.8032786885245902,
 'recall': 0.8032786885245902,
 'support': 122}
weighted
{'f1-score': 0.7332972316578874,
 'precision': 0.840422568836543,
 'recall': 0.8032786885245902,
 'support': 122}
### report_full
macro
{'f1-score': 0.43648839297165576,
 'precision': 0.6677791262135923,
 'recall': 0.428030303030303,
 'support': 191}
micro
{'f1-score': 0.6261980830670926,
 'precision': 0.8032786885245902,
 'recall': 0.5130890052356021,
 'support': 191}
weighted
{'f1-score': 0.5557559727294763,
 'precision': 0.8071849743303005,
 'recall': 0.5130890052356021,
 'support': 191}
```

## javascript
### Summary
2 rules, avg.len. 2.0

| | |
|-|-|
|Min support|183|
|Max support|199|
|Min confidence|0.9972677826881409|
|Max confidence|0.9974874258041382|

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
                     'min_samples_leaf': 94,
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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.997. Support: 199.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles in {KEY}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 183.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 2.0, "max_conf": 0.9974874258041382, "max_support": 199, "min_conf": 0.9972677826881409, "min_support": 183, "num_rules": 2}}
```
</details>
