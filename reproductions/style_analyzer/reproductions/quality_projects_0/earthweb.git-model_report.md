# Model report for file:///tmp/top-repos-quality-repos-mfw870w_/earthweb.git HEAD 18b3ad88d53dba587c19de8de9f13a970d457b2f

### Dump

```json
{'created_at': '2021-09-01 18:12:39',
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
 'size': '19.9 kB',
 'tags': [],
 'uuid': 'abc00dd3-68e8-4420-86b5-416a4712e7d4',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-mfw870w_/earthweb.git 18b3ad88d53dba587c19de8de9f13a970d457b2f

# javascript
33 rules, avg.len. 7.5
## train
PPCR: 0.904174
### report
macro
{'f1-score': 0.6730285671606125,
 'precision': 0.7245113119429533,
 'recall': 0.6415522707260998,
 'support': 56840}
micro
{'f1-score': 0.9446164672765658,
 'precision': 0.9446164672765658,
 'recall': 0.9446164672765658,
 'support': 56840}
weighted
{'f1-score': 0.9411408713338167,
 'precision': 0.9403586477009093,
 'recall': 0.9446164672765658,
 'support': 56840}
### report_full
macro
{'f1-score': 0.5523178662714002,
 'precision': 0.7245113119429533,
 'recall': 0.4951032903021838,
 'support': 62864}
micro
{'f1-score': 0.8970794626745974,
 'precision': 0.9446164672765658,
 'recall': 0.8540977347925681,
 'support': 62864}
weighted
{'f1-score': 0.876426182340493,
 'precision': 0.9328349744644824,
 'recall': 0.8540977347925681,
 'support': 62864}
## test
PPCR: 0.877517
### report
macro
{'f1-score': 0.6035993928389504,
 'precision': 0.6336913123387904,
 'recall': 0.5814937177203537,
 'support': 5710}
micro
{'f1-score': 0.9308231173380035,
 'precision': 0.9308231173380035,
 'recall': 0.9308231173380035,
 'support': 5710}
weighted
{'f1-score': 0.9283380151704679,
 'precision': 0.9295028540245202,
 'recall': 0.9308231173380035,
 'support': 5710}
### report_full
macro
{'f1-score': 0.4927298568245761,
 'precision': 0.6336913123387904,
 'recall': 0.4504948500633283,
 'support': 6507}
micro
{'f1-score': 0.8700990423180814,
 'precision': 0.9308231173380035,
 'recall': 0.8168126632856924,
 'support': 6507}
weighted
{'f1-score': 0.8385819451784475,
 'precision': 0.9247770058004594,
 'recall': 0.8168126632856924,
 'support': 6507}
```

## javascript
### Summary
15 rules, avg.len. 8.5

| | |
|-|-|
|Min support|125|
|Max support|10408|
|Min confidence|0.9217252135276794|
|Max confidence|0.9992017149925232|

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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.980. Support: 10408.` |
| 2 | `  -1.reserved = (<br>	∧ -4.reserved not in {await}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>⇒ y = ∅<br>Confidence: 0.966. Support: 2376.` |
| 3 | `  •••start_line ≤ 192<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.diff_offset ≥ 7<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = "<br>Confidence: 0.944. Support: 188.` |
| 4 | `  -1.diff_col ≤ 13<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 23<br>	∧ ^1.internal_type not in {MemberExpression, ObjectExpression}<br>⇒ y = ∅<br>Confidence: 0.935. Support: 177.` |
| 5 | `  -1.diff_col ≤ 13<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -2.diff_col ≥ 5<br>	∧ -2.reserved = (<br>	∧ +1.length ≤ 22<br>	∧ ^1.internal_type not in {MemberExpression, ObjectExpression}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 125.` |
| 6 | `  •••start_col ≤ 7<br>	∧ -1.diff_col ≤ 13<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ +1.length ≤ 22<br>	∧ ^1.internal_type not in {MemberExpression, ObjectExpression}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 180.` |
| 7 | `  •••start_col ≤ 7<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ +1.length ≤ 22<br>	∧ ^1.internal_type not in {MemberExpression, ObjectExpression}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 157.` |
| 8 | `  +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 1879.` |
| 9 | `  •••start_line ≥ 237<br>	∧ -1.internal_type = StringLiteral<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles in {CALL}<br>⇒ y = '<br>Confidence: 0.988. Support: 363.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 550.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 313.` |
| 12 | `  •••start_line ≥ 228<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = function<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 231.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {function, if}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 8078.` |
| 14 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {CONDITION}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 127.` |
| 15 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {function, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 4816.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.533333333333333, "max_conf": 0.9992017149925232, "max_support": 10408, "min_conf": 0.9217252135276794, "min_support": 125, "num_rules": 15}}
```
</details>
