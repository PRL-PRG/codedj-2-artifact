# Model report for file:///tmp/top-repos-quality-repos-d5axd4fp/dylans-cms.git HEAD d71a957fde2255a444ee13e1adabd4b309fcbffe

### Dump

```json
{'created_at': '2021-09-02 04:59:55',
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
 'size': '16.2 kB',
 'tags': [],
 'uuid': '7f47e534-e142-4452-a0d1-5161482db32c',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-d5axd4fp/dylans-cms.git d71a957fde2255a444ee13e1adabd4b309fcbffe

# javascript
6 rules, avg.len. 7.3
## train
PPCR: 0.568894
### report
macro
{'f1-score': 0.3491892175333299,
 'precision': 0.3478765203940606,
 'recall': 0.35068505902471314,
 'support': 6028}
micro
{'f1-score': 0.9344724618447247,
 'precision': 0.9344724618447247,
 'recall': 0.9344724618447247,
 'support': 6028}
weighted
{'f1-score': 0.9179759020814859,
 'precision': 0.9023615933660533,
 'recall': 0.9344724618447247,
 'support': 6028}
### report_full
macro
{'f1-score': 0.2596741739178767,
 'precision': 0.3478765203940606,
 'recall': 0.22006131577199817,
 'support': 10596}
micro
{'f1-score': 0.6776948989412896,
 'precision': 0.9344724618447247,
 'recall': 0.5316157040392601,
 'support': 10596}
weighted
{'f1-score': 0.6036807472606119,
 'precision': 0.7584438014781915,
 'recall': 0.5316157040392601,
 'support': 10596}
## test
PPCR: 0.586719
### report
macro
{'f1-score': 0.3438130918528254,
 'precision': 0.34036317155558826,
 'recall': 0.34737879457707044,
 'support': 1502}
micro
{'f1-score': 0.9314247669773634,
 'precision': 0.9314247669773635,
 'recall': 0.9314247669773635,
 'support': 1502}
weighted
{'f1-score': 0.9196734124091792,
 'precision': 0.9082553655943164,
 'recall': 0.9314247669773635,
 'support': 1502}
### report_full
macro
{'f1-score': 0.27260636697078783,
 'precision': 0.34036317155558826,
 'recall': 0.24205369524823434,
 'support': 2560}
micro
{'f1-score': 0.688823239783358,
 'precision': 0.9314247669773635,
 'recall': 0.546484375,
 'support': 2560}
weighted
{'f1-score': 0.6146860721716191,
 'precision': 0.7549418465017441,
 'recall': 0.546484375,
 'support': 2560}
```

## javascript
### Summary
3 rules, avg.len. 6.3

| | |
|-|-|
|Min support|207|
|Max support|3509|
|Min confidence|0.9437161684036255|
|Max confidence|0.9982331991195679|

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
| 1 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles not in {INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.998. Support: 283.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = '<br>Confidence: 0.998. Support: 207.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {}}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.diff_offset ≥ 6<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {File, Program}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, STATEMENT}<br>	∧ ^2.roles not in {MAP, TYPE}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 3509.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.333333333333333, "max_conf": 0.9982331991195679, "max_support": 3509, "min_conf": 0.9437161684036255, "min_support": 207, "num_rules": 3}}
```
</details>
