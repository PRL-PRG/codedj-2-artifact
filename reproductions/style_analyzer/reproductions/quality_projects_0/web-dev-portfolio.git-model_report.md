# Model report for file:///tmp/top-repos-quality-repos-d_85bwa5/web-dev-portfolio.git HEAD 072995f16f6bc081ce106652b0966b8431e233c6

### Dump

```json
{'created_at': '2021-09-01 19:53:08',
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
 'size': '14.0 kB',
 'tags': [],
 'uuid': '8188ba63-c518-4f43-b546-6fc9f95c411b',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-d_85bwa5/web-dev-portfolio.git 072995f16f6bc081ce106652b0966b8431e233c6

# javascript
16 rules, avg.len. 5.4
## train
PPCR: 0.452416
### report
macro
{'f1-score': 0.5567978743660313,
 'precision': 0.550716998005927,
 'recall': 0.563476042304145,
 'support': 2144}
micro
{'f1-score': 0.9612873134328358,
 'precision': 0.9612873134328358,
 'recall': 0.9612873134328358,
 'support': 2144}
weighted
{'f1-score': 0.950995826984282,
 'precision': 0.9411034497126153,
 'recall': 0.9612873134328358,
 'support': 2144}
### report_full
macro
{'f1-score': 0.29094808471902356,
 'precision': 0.550716998005927,
 'recall': 0.21515068031359444,
 'support': 4739}
micro
{'f1-score': 0.5988667732093563,
 'precision': 0.9612873134328358,
 'recall': 0.43490187803334035,
 'support': 4739}
weighted
{'f1-score': 0.5467605662381293,
 'precision': 0.8442739367032157,
 'recall': 0.43490187803334035,
 'support': 4739}
## test
PPCR: 0.578781
### report
macro
{'f1-score': 0.5739560316342993,
 'precision': 0.5818447229578962,
 'recall': 0.5691446540880503,
 'support': 731}
micro
{'f1-score': 0.9794801641586868,
 'precision': 0.9794801641586868,
 'recall': 0.9794801641586868,
 'support': 731}
weighted
{'f1-score': 0.9758191350291865,
 'precision': 0.9734055886858892,
 'recall': 0.9794801641586868,
 'support': 731}
### report_full
macro
{'f1-score': 0.34734578742237887,
 'precision': 0.5818447229578962,
 'recall': 0.27485431861935095,
 'support': 1263}
micro
{'f1-score': 0.7181544633901705,
 'precision': 0.9794801641586868,
 'recall': 0.566904196357878,
 'support': 1263}
weighted
{'f1-score': 0.6621189073007416,
 'precision': 0.915082759760196,
 'recall': 0.566904196357878,
 'support': 1263}
```

## javascript
### Summary
10 rules, avg.len. 4.5

| | |
|-|-|
|Min support|137|
|Max support|1073|
|Min confidence|0.9475524425506592|
|Max confidence|0.9986486434936523|

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
                     'max_depth': 10,
                     'min_samples_leaf': 90,
                     'min_samples_split': 183,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 1073.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {IDENTIFIER} and not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 376.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {QUALIFIED} and not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 399.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ +1.roles in {LITERAL}<br>⇒ y = "<br>Confidence: 0.961. Support: 142.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 371.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 998.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 376.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {LITERAL}<br>	∧ ^2.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.948. Support: 143.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {IDENTIFIER} and not in {DECLARATION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 370.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {LITERAL}<br>	∧ ^1.internal_type = JSXAttribute<br>⇒ y = "<br>Confidence: 0.953. Support: 137.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.5, "max_conf": 0.9986486434936523, "max_support": 1073, "min_conf": 0.9475524425506592, "min_support": 137, "num_rules": 10}}
```
</details>
