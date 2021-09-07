# Model report for file:///tmp/top-repos-quality-repos-t35jwtlp/sauti-studio-fe-v2.git HEAD 9ab26b97a4990879526f2949933e5866f3033d2f

### Dump

```json
{'created_at': '2021-09-02 11:35:14',
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
 'size': '16.6 kB',
 'tags': [],
 'uuid': '78686016-8482-410e-b8f3-17a1bfc65e7c',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-t35jwtlp/sauti-studio-fe-v2.git 9ab26b97a4990879526f2949933e5866f3033d2f

# javascript
14 rules, avg.len. 5.9
## train
PPCR: 0.780206
### report
macro
{'f1-score': 0.5293501003059528,
 'precision': 0.5545336695944498,
 'recall': 0.5089651147019548,
 'support': 7284}
micro
{'f1-score': 0.9100768808347062,
 'precision': 0.9100768808347062,
 'recall': 0.9100768808347062,
 'support': 7284}
weighted
{'f1-score': 0.9017081257291107,
 'precision': 0.8953164001836015,
 'recall': 0.9100768808347062,
 'support': 7284}
### report_full
macro
{'f1-score': 0.43222966476660063,
 'precision': 0.5545336695944498,
 'recall': 0.36801392284106005,
 'support': 9336}
micro
{'f1-score': 0.7977135980746088,
 'precision': 0.9100768808347062,
 'recall': 0.7100471293916024,
 'support': 9336}
weighted
{'f1-score': 0.7649765474781249,
 'precision': 0.8557048209624704,
 'recall': 0.7100471293916024,
 'support': 9336}
## test
PPCR: 0.759828
### report
macro
{'f1-score': 0.5281134675603777,
 'precision': 0.5596457982406887,
 'recall': 0.504864737747529,
 'support': 1063}
micro
{'f1-score': 0.8692380056444027,
 'precision': 0.8692380056444027,
 'recall': 0.8692380056444027,
 'support': 1063}
weighted
{'f1-score': 0.8671735167075733,
 'precision': 0.8718762154225758,
 'recall': 0.8692380056444027,
 'support': 1063}
### report_full
macro
{'f1-score': 0.4116257813384556,
 'precision': 0.5596457982406887,
 'recall': 0.3587328829234957,
 'support': 1399}
micro
{'f1-score': 0.7506092607636068,
 'precision': 0.8692380056444027,
 'recall': 0.6604717655468192,
 'support': 1399}
weighted
{'f1-score': 0.7116606478469313,
 'precision': 0.8277899496940209,
 'recall': 0.6604717655468192,
 'support': 1399}
```

## javascript
### Summary
6 rules, avg.len. 4.2

| | |
|-|-|
|Min support|103|
|Max support|564|
|Min confidence|0.9271844625473022|
|Max confidence|0.9991135001182556|

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
| 1 | `  +1.length ≥ 2<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 564.` |
| 2 | `  +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 326.` |
| 3 | `  +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 196.` |
| 4 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.930. Support: 178.` |
| 5 | `  -1.internal_type = StringLiteral<br>	∧ -5.diff_col ≥ 23<br>	∧ +1.length ≤ 1<br>⇒ y = "<br>Confidence: 0.927. Support: 103.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {BINARY, QUALIFIED} and not in {DECLARATION, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 103.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.166666666666667, "max_conf": 0.9991135001182556, "max_support": 564, "min_conf": 0.9271844625473022, "min_support": 103, "num_rules": 6}}
```
</details>
