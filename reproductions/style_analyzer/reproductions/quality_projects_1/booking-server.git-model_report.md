# Model report for file:///tmp/top-repos-quality-repos-g_8x5w7c/booking-server.git HEAD 79f5916a3295e18ce614e96eb890a456673b2b77

### Dump

```json
{'created_at': '2021-09-02 01:26:15',
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
 'size': '16.8 kB',
 'tags': [],
 'uuid': 'f70ae959-aa70-46c3-bef5-0009445da5d7',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-g_8x5w7c/booking-server.git 79f5916a3295e18ce614e96eb890a456673b2b77

# javascript
9 rules, avg.len. 4.7
## train
PPCR: 0.669368
### report
macro
{'f1-score': 0.6748457509777539,
 'precision': 0.6750232929824351,
 'recall': 0.6777678316911818,
 'support': 4956}
micro
{'f1-score': 0.9521791767554479,
 'precision': 0.9521791767554479,
 'recall': 0.9521791767554479,
 'support': 4956}
weighted
{'f1-score': 0.9444028321126651,
 'precision': 0.9379374468915611,
 'recall': 0.9521791767554479,
 'support': 4956}
### report_full
macro
{'f1-score': 0.5047669207820655,
 'precision': 0.6750232929824351,
 'recall': 0.45307499558188014,
 'support': 7404}
micro
{'f1-score': 0.7635922330097087,
 'precision': 0.9521791767554479,
 'recall': 0.6373581847649918,
 'support': 7404}
weighted
{'f1-score': 0.6965738660929642,
 'precision': 0.9045817345923007,
 'recall': 0.6373581847649918,
 'support': 7404}
## test
PPCR: 0.724747
### report
macro
{'f1-score': 0.6774568830014623,
 'precision': 0.7050296195033037,
 'recall': 0.6588093891402715,
 'support': 287}
micro
{'f1-score': 0.9616724738675958,
 'precision': 0.9616724738675958,
 'recall': 0.9616724738675958,
 'support': 287}
weighted
{'f1-score': 0.9584058971366274,
 'precision': 0.9591797040907624,
 'recall': 0.9616724738675958,
 'support': 287}
### report_full
macro
{'f1-score': 0.5345818847673555,
 'precision': 0.7050296195033037,
 'recall': 0.47053586233647793,
 'support': 396}
micro
{'f1-score': 0.8081991215226941,
 'precision': 0.9616724738675958,
 'recall': 0.696969696969697,
 'support': 396}
weighted
{'f1-score': 0.7621618624225537,
 'precision': 0.9109913212784025,
 'recall': 0.696969696969697,
 'support': 396}
```

## javascript
### Summary
6 rules, avg.len. 5.2

| | |
|-|-|
|Min support|178|
|Max support|1231|
|Min confidence|0.9220430254936218|
|Max confidence|0.9895730018615723|

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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.959. Support: 1231.` |
| 2 | `  -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 256.` |
| 3 | `  -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.922. Support: 186.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 178.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 1007.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 569.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.166666666666667, "max_conf": 0.9895730018615723, "max_support": 1231, "min_conf": 0.9220430254936218, "min_support": 178, "num_rules": 6}}
```
</details>
