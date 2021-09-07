# Model report for file:///tmp/top-repos-quality-repos-0z55eys2/jirascope.git HEAD 657724f9c2556e89f886483e51e27277cbb17a64

### Dump

```json
{'created_at': '2021-09-01 03:35:10',
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
 'size': '15.9 kB',
 'tags': [],
 'uuid': '86f92be0-04bc-4826-85cc-f6684c63e680',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-0z55eys2/jirascope.git 657724f9c2556e89f886483e51e27277cbb17a64

# javascript
9 rules, avg.len. 6.2
## train
PPCR: 0.762255
### report
macro
{'f1-score': 0.6316632971898046,
 'precision': 0.639388332334218,
 'recall': 0.6298792190046416,
 'support': 7573}
micro
{'f1-score': 0.9416347550508385,
 'precision': 0.9416347550508385,
 'recall': 0.9416347550508385,
 'support': 7573}
weighted
{'f1-score': 0.937636229713919,
 'precision': 0.9355864224596268,
 'recall': 0.9416347550508385,
 'support': 7573}
### report_full
macro
{'f1-score': 0.5117503255417508,
 'precision': 0.639388332334218,
 'recall': 0.44491637550353236,
 'support': 9935}
micro
{'f1-score': 0.8145990404386566,
 'precision': 0.9416347550508385,
 'recall': 0.7177654755913437,
 'support': 9935}
weighted
{'f1-score': 0.7859527349399986,
 'precision': 0.8882206012872348,
 'recall': 0.7177654755913437,
 'support': 9935}
## test
PPCR: 0.774151
### report
macro
{'f1-score': 0.6188836415454654,
 'precision': 0.6251238545562624,
 'recall': 0.6205065998893607,
 'support': 1779}
micro
{'f1-score': 0.9314221472737493,
 'precision': 0.9314221472737493,
 'recall': 0.9314221472737493,
 'support': 1779}
weighted
{'f1-score': 0.9248556680345416,
 'precision': 0.9210183391904303,
 'recall': 0.9314221472737493,
 'support': 1779}
### report_full
macro
{'f1-score': 0.503970969505134,
 'precision': 0.6251238545562624,
 'recall': 0.4465402438957957,
 'support': 2298}
micro
{'f1-score': 0.8128525876870247,
 'precision': 0.9314221472737493,
 'recall': 0.7210617928633595,
 'support': 2298}
weighted
{'f1-score': 0.7805264865013689,
 'precision': 0.8714661718470726,
 'recall': 0.7210617928633595,
 'support': 2298}
```

## javascript
### Summary
5 rules, avg.len. 5.0

| | |
|-|-|
|Min support|136|
|Max support|2161|
|Min confidence|0.9396551847457886|
|Max confidence|0.9986910820007324|

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
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 2161.` |
| 2 | `  -1.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {CALL} and not in {FUNCTION, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 1404.` |
| 3 | `  -1.roles not in {LITERAL}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {CALL, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 382.` |
| 4 | `  -1.reserved = {<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.roles not in {CALL, IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.940. Support: 174.` |
| 5 | `  -1.reserved not in {), {}<br>	∧ -1.roles not in {MAP}<br>	∧ -3.diff_line = 0<br>	∧ +1.reserved not in {), }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {BLOCK} and not in {CALL, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 136.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.0, "max_conf": 0.9986910820007324, "max_support": 2161, "min_conf": 0.9396551847457886, "min_support": 136, "num_rules": 5}}
```
</details>
