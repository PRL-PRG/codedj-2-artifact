# Model report for file:///tmp/top-repos-quality-repos-5n14cu3_/todo-web.git HEAD 2c05c6919a24b3e8ca27a0604c85a084fbfc86b4

### Dump

```json
{'created_at': '2021-09-02 00:52:32',
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
 'size': '15.5 kB',
 'tags': [],
 'uuid': '73186209-d84d-46d8-bf67-68b55e92f45f',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-5n14cu3_/todo-web.git 2c05c6919a24b3e8ca27a0604c85a084fbfc86b4

# javascript
11 rules, avg.len. 5.0
## train
PPCR: 0.767893
### report
macro
{'f1-score': 0.40089490968445346,
 'precision': 0.39537632063909933,
 'recall': 0.4067066129042186,
 'support': 4152}
micro
{'f1-score': 0.9116088631984586,
 'precision': 0.9116088631984586,
 'recall': 0.9116088631984586,
 'support': 4152}
weighted
{'f1-score': 0.8928960334260694,
 'precision': 0.8751762075293893,
 'recall': 0.9116088631984586,
 'support': 4152}
### report_full
macro
{'f1-score': 0.3785991653537447,
 'precision': 0.39537632063909933,
 'recall': 0.36395907010054795,
 'support': 5407}
micro
{'f1-score': 0.7919238414060049,
 'precision': 0.9116088631984586,
 'recall': 0.7000184945441095,
 'support': 5407}
weighted
{'f1-score': 0.7345802177134589,
 'precision': 0.7737925116969004,
 'recall': 0.7000184945441095,
 'support': 5407}
## test
PPCR: 0.831718
### report
macro
{'f1-score': 0.37001875130217377,
 'precision': 0.3591971427322623,
 'recall': 0.3831624982916496,
 'support': 944}
micro
{'f1-score': 0.7934322033898306,
 'precision': 0.7934322033898306,
 'recall': 0.7934322033898306,
 'support': 944}
weighted
{'f1-score': 0.7652190123843345,
 'precision': 0.7416985202550483,
 'recall': 0.7934322033898306,
 'support': 944}
### report_full
macro
{'f1-score': 0.3539218371764146,
 'precision': 0.3591971427322623,
 'recall': 0.3488977367238237,
 'support': 1135}
micro
{'f1-score': 0.7205387205387206,
 'precision': 0.7934322033898306,
 'recall': 0.6599118942731278,
 'support': 1135}
weighted
{'f1-score': 0.6746168626965784,
 'precision': 0.6900542848192165,
 'recall': 0.6599118942731278,
 'support': 1135}
```

## javascript
### Summary
7 rules, avg.len. 3.9

| | |
|-|-|
|Min support|132|
|Max support|898|
|Min confidence|0.921800971031189|
|Max confidence|0.9981481432914734|

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
                     'min_samples_leaf': 103,
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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.998. Support: 270.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.996. Support: 132.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.974. Support: 133.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.951. Support: 133.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 898.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {QUALIFIED} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.939. Support: 352.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 211.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.857142857142857, "max_conf": 0.9981481432914734, "max_support": 898, "min_conf": 0.921800971031189, "min_support": 132, "num_rules": 7}}
```
</details>
