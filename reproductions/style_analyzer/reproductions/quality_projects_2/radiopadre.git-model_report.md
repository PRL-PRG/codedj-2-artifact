# Model report for file:///tmp/top-repos-quality-repos-iuygsz4o/radiopadre.git HEAD 3bf934eba69144d9707777a57da0e827625517a3

### Dump

```json
{'created_at': '2021-09-02 04:42:16',
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
 'size': '13.5 kB',
 'tags': [],
 'uuid': '71ca292f-f0f1-4673-a25e-040069f628d8',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-iuygsz4o/radiopadre.git 3bf934eba69144d9707777a57da0e827625517a3

# javascript
2 rules, avg.len. 2.0
## train
PPCR: 0.391095
### report
macro
{'f1-score': 0.38020063940028664,
 'precision': 0.3690677966101695,
 'recall': 0.39362244897959187,
 'support': 975}
micro
{'f1-score': 0.961025641025641,
 'precision': 0.961025641025641,
 'recall': 0.961025641025641,
 'support': 975}
weighted
{'f1-score': 0.9553691815845934,
 'precision': 0.9519483167853442,
 'recall': 0.961025641025641,
 'support': 975}
### report_full
macro
{'f1-score': 0.2397567883241858,
 'precision': 0.3690677966101695,
 'recall': 0.17870690895548919,
 'support': 2493}
micro
{'f1-score': 0.5403690888119954,
 'precision': 0.961025641025641,
 'recall': 0.3758523866827116,
 'support': 2493}
weighted
{'f1-score': 0.49468503955534765,
 'precision': 0.729806038393813,
 'recall': 0.3758523866827116,
 'support': 2493}
## test
PPCR: 0.403743
### report
macro
{'f1-score': 0.3757790893034076,
 'precision': 0.3640326975476839,
 'recall': 0.3912737508796622,
 'support': 1726}
micro
{'f1-score': 0.9617612977983777,
 'precision': 0.9617612977983777,
 'recall': 0.9617612977983777,
 'support': 1726}
weighted
{'f1-score': 0.9620910267296878,
 'precision': 0.966320515532598,
 'recall': 0.9617612977983777,
 'support': 1726}
### report_full
macro
{'f1-score': 0.22380858838735027,
 'precision': 0.3640326975476839,
 'recall': 0.16204991087344028,
 'support': 4275}
micro
{'f1-score': 0.5532411264789203,
 'precision': 0.9617612977983777,
 'recall': 0.3883040935672515,
 'support': 4275}
weighted
{'f1-score': 0.5290749450678645,
 'precision': 0.8334107748936374,
 'recall': 0.3883040935672515,
 'support': 4275}
```

## javascript
### Summary
1 rules, avg.len. 1.0

| | |
|-|-|
|Min support|621|
|Max support|621|
|Min confidence|0.987922728061676|
|Max confidence|0.987922728061676|

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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.988. Support: 621.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 1.0, "max_conf": 0.987922728061676, "max_support": 621, "min_conf": 0.987922728061676, "min_support": 621, "num_rules": 1}}
```
</details>
