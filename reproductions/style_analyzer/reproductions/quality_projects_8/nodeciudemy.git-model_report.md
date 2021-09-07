# Model report for file:///tmp/top-repos-quality-repos-tirec064/nodeciudemy.git HEAD 20606fce8a2305e2424235578ed09985e6642270

### Dump

```json
{'created_at': '2021-09-01 01:04:48',
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
 'size': '15.1 kB',
 'tags': [],
 'uuid': '63bccb56-d2e8-452f-812b-336999d17ecf',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-tirec064/nodeciudemy.git 20606fce8a2305e2424235578ed09985e6642270

# javascript
4 rules, avg.len. 4.0
## train
PPCR: 0.527859
### report
macro
{'f1-score': 0.27447175526414525,
 'precision': 0.26708697521134833,
 'recall': 0.28317836010143704,
 'support': 2160}
micro
{'f1-score': 0.8796296296296297,
 'precision': 0.8796296296296297,
 'recall': 0.8796296296296297,
 'support': 2160}
weighted
{'f1-score': 0.8243301346154303,
 'precision': 0.7763433819978454,
 'recall': 0.8796296296296297,
 'support': 2160}
### report_full
macro
{'f1-score': 0.2173249782909714,
 'precision': 0.26708697521134833,
 'recall': 0.19144518272425248,
 'support': 4092}
micro
{'f1-score': 0.6078055022392834,
 'precision': 0.8796296296296297,
 'recall': 0.46432062561094817,
 'support': 4092}
weighted
{'f1-score': 0.4851526441058866,
 'precision': 0.5197635918508213,
 'recall': 0.46432062561094817,
 'support': 4092}
## test
PPCR: 0.571656
### report
macro
{'f1-score': 0.2685137346077671,
 'precision': 0.26632954066582387,
 'recall': 0.27272727272727276,
 'support': 359}
micro
{'f1-score': 0.871866295264624,
 'precision': 0.871866295264624,
 'recall': 0.871866295264624,
 'support': 359}
weighted
{'f1-score': 0.8151154083155775,
 'precision': 0.7666904955587874,
 'recall': 0.871866295264624,
 'support': 359}
### report_full
macro
{'f1-score': 0.2169158361018826,
 'precision': 0.26632954066582387,
 'recall': 0.1913630781825624,
 'support': 628}
micro
{'f1-score': 0.6342451874366768,
 'precision': 0.871866295264624,
 'recall': 0.4984076433121019,
 'support': 628}
weighted
{'f1-score': 0.5158047573199032,
 'precision': 0.5440176239595663,
 'recall': 0.4984076433121019,
 'support': 628}
```

## javascript
### Summary
3 rules, avg.len. 4.0

| | |
|-|-|
|Min support|127|
|Max support|184|
|Min confidence|0.9960629940032959|
|Max confidence|0.9972826242446899|

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
                     'min_samples_leaf': 120,
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
| 1 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.996. Support: 139.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 184.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 127.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.0, "max_conf": 0.9972826242446899, "max_support": 184, "min_conf": 0.9960629940032959, "min_support": 127, "num_rules": 3}}
```
</details>
