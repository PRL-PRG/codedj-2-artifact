# Model report for file:///tmp/top-repos-quality-repos-4boubbal/recipe-app.git HEAD ee8134081c1554b85305271fdfafc8c7dd542381

### Dump

```json
{'created_at': '2021-09-01 20:06:41',
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
 'size': '15.8 kB',
 'tags': [],
 'uuid': 'fc1c0a7f-578b-44c7-a7b3-9501bcd4f06e',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-4boubbal/recipe-app.git ee8134081c1554b85305271fdfafc8c7dd542381

# javascript
6 rules, avg.len. 5.3
## train
PPCR: 0.724088
### report
macro
{'f1-score': 0.48321613319577633,
 'precision': 0.5162401134252351,
 'recall': 0.4604363758920246,
 'support': 3514}
micro
{'f1-score': 0.8870233352305066,
 'precision': 0.8870233352305066,
 'recall': 0.8870233352305066,
 'support': 3514}
weighted
{'f1-score': 0.8726475878214128,
 'precision': 0.873103987106449,
 'recall': 0.8870233352305066,
 'support': 3514}
### report_full
macro
{'f1-score': 0.41577447453100885,
 'precision': 0.5162401134252351,
 'recall': 0.36697820640099027,
 'support': 4853}
micro
{'f1-score': 0.7450699175331659,
 'precision': 0.8870233352305066,
 'recall': 0.6422831238409231,
 'support': 4853}
weighted
{'f1-score': 0.6886818620182411,
 'precision': 0.8011857558731622,
 'recall': 0.6422831238409231,
 'support': 4853}
## test
PPCR: 0.630893
### report
macro
{'f1-score': 0.44843887842909375,
 'precision': 0.49009762458829736,
 'recall': 0.4279904626425715,
 'support': 629}
micro
{'f1-score': 0.848966613672496,
 'precision': 0.848966613672496,
 'recall': 0.848966613672496,
 'support': 629}
weighted
{'f1-score': 0.8287629744426792,
 'precision': 0.8340700192487089,
 'recall': 0.848966613672496,
 'support': 629}
### report_full
macro
{'f1-score': 0.36344184078804437,
 'precision': 0.49009762458829736,
 'recall': 0.32001805786836474,
 'support': 997}
micro
{'f1-score': 0.6568265682656828,
 'precision': 0.848966613672496,
 'recall': 0.5356068204613842,
 'support': 997}
weighted
{'f1-score': 0.5982858224173082,
 'precision': 0.7656662561794546,
 'recall': 0.5356068204613842,
 'support': 997}
```

## javascript
### Summary
1 rules, avg.len. 1.0

| | |
|-|-|
|Min support|206|
|Max support|206|
|Min confidence|0.9927184581756592|
|Max confidence|0.9927184581756592|

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
| 1 | `  -1.reserved = :<br>⇒ y = ␣<br>Confidence: 0.993. Support: 206.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 1.0, "max_conf": 0.9927184581756592, "max_support": 206, "min_conf": 0.9927184581756592, "min_support": 206, "num_rules": 1}}
```
</details>
