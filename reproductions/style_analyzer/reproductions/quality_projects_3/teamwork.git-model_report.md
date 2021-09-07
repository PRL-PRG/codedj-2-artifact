# Model report for file:///tmp/top-repos-quality-repos-rnv6mo0r/teamwork.git HEAD deba9917cadbe438797e15502a82ffa5a671edbb

### Dump

```json
{'created_at': '2021-09-02 08:19:00',
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
 'size': '15.6 kB',
 'tags': [],
 'uuid': '6a9e364f-d982-41b3-b98a-e43161e45a95',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-rnv6mo0r/teamwork.git deba9917cadbe438797e15502a82ffa5a671edbb

# javascript
6 rules, avg.len. 5.0
## train
PPCR: 0.644068
### report
macro
{'f1-score': 0.4842570327135077,
 'precision': 0.5294931069934952,
 'recall': 0.46003616253156976,
 'support': 3990}
micro
{'f1-score': 0.9318295739348371,
 'precision': 0.9318295739348371,
 'recall': 0.9318295739348371,
 'support': 3990}
weighted
{'f1-score': 0.9199127722960656,
 'precision': 0.9192483608462234,
 'recall': 0.9318295739348371,
 'support': 3990}
### report_full
macro
{'f1-score': 0.3375775250588532,
 'precision': 0.5294931069934952,
 'recall': 0.2844009070511995,
 'support': 6195}
micro
{'f1-score': 0.7300932744231713,
 'precision': 0.9318295739348371,
 'recall': 0.6001614205004036,
 'support': 6195}
weighted
{'f1-score': 0.6515989118802852,
 'precision': 0.8506105050318132,
 'recall': 0.6001614205004036,
 'support': 6195}
## test
PPCR: 0.580308
### report
macro
{'f1-score': 0.42521333512892145,
 'precision': 0.4802855317658978,
 'recall': 0.3981348566276687,
 'support': 943}
micro
{'f1-score': 0.8504772004241782,
 'precision': 0.8504772004241782,
 'recall': 0.8504772004241782,
 'support': 943}
weighted
{'f1-score': 0.8262586547446135,
 'precision': 0.8325405569483947,
 'recall': 0.8504772004241782,
 'support': 943}
### report_full
macro
{'f1-score': 0.2833971171567079,
 'precision': 0.4802855317658978,
 'recall': 0.229505042023809,
 'support': 1625}
micro
{'f1-score': 0.6246105919003114,
 'precision': 0.8504772004241782,
 'recall': 0.49353846153846154,
 'support': 1625}
weighted
{'f1-score': 0.5422536652235648,
 'precision': 0.7534662438625205,
 'recall': 0.49353846153846154,
 'support': 1625}
```

## javascript
### Summary
3 rules, avg.len. 4.7

| | |
|-|-|
|Min support|123|
|Max support|1431|
|Min confidence|0.9570230841636658|
|Max confidence|0.996268630027771|

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
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 1431.` |
| 2 | `  -2.diff_offset ≤ 17<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 3<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 134.` |
| 3 | `  -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≤ 17<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 3<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 123.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.666666666666667, "max_conf": 0.996268630027771, "max_support": 1431, "min_conf": 0.9570230841636658, "min_support": 123, "num_rules": 3}}
```
</details>
