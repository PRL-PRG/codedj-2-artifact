# Model report for file:///tmp/top-repos-quality-repos-8bmdcf1m/rona-v2.git HEAD a56dca26de054977783196add322b6b8c2f622a8

### Dump

```json
{'created_at': '2021-09-01 03:31:43',
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
 'size': '13.3 kB',
 'tags': [],
 'uuid': 'a6a5e4aa-e86f-406e-a65c-00933fd471d0',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-8bmdcf1m/rona-v2.git a56dca26de054977783196add322b6b8c2f622a8

# javascript
1 rules, avg.len. 1.0
## train
PPCR: 0.154066
### report
macro
{'f1-score': 0.1975,
 'precision': 0.19506172839506172,
 'recall': 0.2,
 'support': 324}
micro
{'f1-score': 0.9753086419753086,
 'precision': 0.9753086419753086,
 'recall': 0.9753086419753086,
 'support': 324}
weighted
{'f1-score': 0.9631172839506174,
 'precision': 0.9512269471117207,
 'recall': 0.9753086419753086,
 'support': 324}
### report_full
macro
{'f1-score': 0.08845346396081175,
 'precision': 0.19506172839506172,
 'recall': 0.0571945701357466,
 'support': 2103}
micro
{'f1-score': 0.2604037906880923,
 'precision': 0.9753086419753086,
 'recall': 0.15026153114598192,
 'support': 2103}
weighted
{'f1-score': 0.2323848732208678,
 'precision': 0.5124660244330557,
 'recall': 0.15026153114598192,
 'support': 2103}
## test
PPCR: 0.095041
### report
macro
{'f1-score': 0.19555555555555554,
 'precision': 0.19130434782608696,
 'recall': 0.2,
 'support': 46}
micro
{'f1-score': 0.9565217391304348,
 'precision': 0.9565217391304348,
 'recall': 0.9565217391304348,
 'support': 46}
weighted
{'f1-score': 0.9352657004830917,
 'precision': 0.9149338374291116,
 'recall': 0.9565217391304348,
 'support': 46}
### report_full
macro
{'f1-score': 0.04902506963788301,
 'precision': 0.19130434782608696,
 'recall': 0.028115015974440893,
 'support': 484}
micro
{'f1-score': 0.16603773584905662,
 'precision': 0.9565217391304348,
 'recall': 0.09090909090909091,
 'support': 484}
weighted
{'f1-score': 0.158521144593568,
 'precision': 0.6185770750988143,
 'recall': 0.09090909090909091,
 'support': 484}
```

## javascript
### Summary
1 rules, avg.len. 1.0

| | |
|-|-|
|Min support|257|
|Max support|257|
|Min confidence|0.966926097869873|
|Max confidence|0.966926097869873|

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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.967. Support: 257.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 1.0, "max_conf": 0.966926097869873, "max_support": 257, "min_conf": 0.966926097869873, "min_support": 257, "num_rules": 1}}
```
</details>
