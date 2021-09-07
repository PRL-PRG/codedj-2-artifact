# Model report for file:///tmp/top-repos-quality-repos-87h5j5he/angular-syca.git HEAD c01051c2e780dd1823d7bf1e8ccbf15947a73c18

### Dump

```json
{'created_at': '2021-09-02 00:10:01',
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
 'size': '14.5 kB',
 'tags': [],
 'uuid': 'cac8a93f-198b-4d19-8a45-35650d3def23',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-87h5j5he/angular-syca.git c01051c2e780dd1823d7bf1e8ccbf15947a73c18

# javascript
3 rules, avg.len. 3.7
## train
PPCR: 0.684953
### report
macro
{'f1-score': 0.3983929091885405,
 'precision': 0.41834322901417426,
 'recall': 0.3996154314420168,
 'support': 2135}
micro
{'f1-score': 0.8627634660421546,
 'precision': 0.8627634660421546,
 'recall': 0.8627634660421546,
 'support': 2135}
weighted
{'f1-score': 0.8221604856260929,
 'precision': 0.8007632906581352,
 'recall': 0.8627634660421546,
 'support': 2135}
### report_full
macro
{'f1-score': 0.29453605140579514,
 'precision': 0.41834322901417426,
 'recall': 0.25642966154411406,
 'support': 3117}
micro
{'f1-score': 0.7014470677837016,
 'precision': 0.8627634660421546,
 'recall': 0.5909528392685275,
 'support': 3117}
weighted
{'f1-score': 0.6260731438639905,
 'precision': 0.7597457574650414,
 'recall': 0.5909528392685275,
 'support': 3117}
## test
PPCR: 0.778082
### report
macro
{'f1-score': 0.419020087046382,
 'precision': 0.43999779905359304,
 'recall': 0.407843137254902,
 'support': 284}
micro
{'f1-score': 0.8485915492957746,
 'precision': 0.8485915492957746,
 'recall': 0.8485915492957746,
 'support': 284}
weighted
{'f1-score': 0.8054828655124157,
 'precision': 0.777499058397928,
 'recall': 0.8485915492957746,
 'support': 284}
### report_full
macro
{'f1-score': 0.3374700427216839,
 'precision': 0.43999779905359304,
 'recall': 0.2896761803011803,
 'support': 365}
micro
{'f1-score': 0.7426810477657936,
 'precision': 0.8485915492957746,
 'recall': 0.6602739726027397,
 'support': 365}
weighted
{'f1-score': 0.6939014332389831,
 'precision': 0.7755451096026086,
 'recall': 0.6602739726027397,
 'support': 365}
```

## javascript
### Summary
0 rules, avg.len. 0.0

| | |
|-|-|
|Min support|1000000000000|
|Max support|-1|
|Min confidence|1|
|Max confidence|0|

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

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 0, "max_conf": 0, "max_support": -1, "min_conf": 1, "min_support": 1000000000000, "num_rules": 0}}
```
</details>
