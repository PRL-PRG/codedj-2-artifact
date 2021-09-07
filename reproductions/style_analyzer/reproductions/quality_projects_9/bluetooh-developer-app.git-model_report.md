# Model report for file:///tmp/top-repos-quality-repos-hgyxie03/bluetooh-developer-app.git HEAD e27b373ff68a87dc91fbda18feb6a33aa4211d6d

### Dump

```json
{'created_at': '2021-08-31 21:26:08',
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
 'size': '21.5 kB',
 'tags': [],
 'uuid': '7555ee2b-8905-4066-bc5f-5ecc5c2648d3',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-hgyxie03/bluetooh-developer-app.git e27b373ff68a87dc91fbda18feb6a33aa4211d6d

# javascript
35 rules, avg.len. 7.1
## train
PPCR: 0.908794
### report
macro
{'f1-score': 0.7716120211333752,
 'precision': 0.8172994089853695,
 'recall': 0.7398446010815555,
 'support': 80351}
micro
{'f1-score': 0.9665592214160371,
 'precision': 0.9665592214160371,
 'recall': 0.9665592214160371,
 'support': 80351}
weighted
{'f1-score': 0.9655920998838682,
 'precision': 0.9656243117992637,
 'recall': 0.9665592214160371,
 'support': 80351}
### report_full
macro
{'f1-score': 0.6153921544107456,
 'precision': 0.8172994089853695,
 'recall': 0.5321146704419388,
 'support': 88415}
micro
{'f1-score': 0.9203749570411102,
 'precision': 0.9665592214160371,
 'recall': 0.8784029859186789,
 'support': 88415}
weighted
{'f1-score': 0.9061066727726844,
 'precision': 0.9582818505731497,
 'recall': 0.8784029859186789,
 'support': 88415}
## test
PPCR: 0.858848
### report
macro
{'f1-score': 0.5701492520584296,
 'precision': 0.6197888355526687,
 'recall': 0.5866661677630515,
 'support': 14317}
micro
{'f1-score': 0.8730879374170566,
 'precision': 0.8730879374170566,
 'recall': 0.8730879374170566,
 'support': 14317}
weighted
{'f1-score': 0.8704802296658363,
 'precision': 0.8790620444266107,
 'recall': 0.8730879374170566,
 'support': 14317}
### report_full
macro
{'f1-score': 0.44605881724845403,
 'precision': 0.6197888355526687,
 'recall': 0.4072794574897278,
 'support': 16670}
micro
{'f1-score': 0.8067899441701358,
 'precision': 0.8730879374170566,
 'recall': 0.7498500299940012,
 'support': 16670}
weighted
{'f1-score': 0.7738129718168298,
 'precision': 0.8537248110518765,
 'recall': 0.7498500299940012,
 'support': 16670}
```

## javascript
### Summary
19 rules, avg.len. 6.5

| | |
|-|-|
|Min support|91|
|Max support|21382|
|Min confidence|0.9234327077865601|
|Max confidence|0.9995791912078857|

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
                     'min_samples_split': 190,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.roles not in {STRING}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 1.000. Support: 15447.` |
| 2 | `  -1.reserved = (<br>	∧ -2.diff_offset ≤ 3<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = "<br>Confidence: 0.973. Support: 245.` |
| 3 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 4224.` |
| 4 | `  -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>⇒ y = ∅<br>Confidence: 0.988. Support: 608.` |
| 5 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>⇒ y = ⏎<br>Confidence: 0.971. Support: 471.` |
| 6 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.length ≥ 10<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>⇒ y = ∅<br>Confidence: 0.996. Support: 136.` |
| 7 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.diff_col ≥ 5<br>	∧ -5.diff_offset ≥ 2<br>	∧ +1.length ≥ 2<br>	∧ +4.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {File, MemberExpression, VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 6230.` |
| 8 | `  +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 2293.` |
| 9 | `  +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 2032.` |
| 10 | `  -1.internal_type = StringLiteral<br>	∧ -3.length ≥ 4<br>	∧ -5.reserved = =<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.966. Support: 191.` |
| 11 | `  -1.internal_type = StringLiteral<br>	∧ -3.diff_col ≥ 19<br>	∧ -3.length ≤ 3<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≥ 5<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = "<br>Confidence: 0.961. Support: 523.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +3.roles in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 172.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = )<br>	∧ +3.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.923. Support: 973.` |
| 14 | `  -1.internal_type = BooleanLiteral<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {)}<br>	∧ +3.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 91.` |
| 15 | `  -1.internal_type not in {BooleanLiteral, StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {)}<br>	∧ +3.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles in {RIGHT}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.924. Support: 138.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.948. Support: 299.` |
| 17 | `  •••start_col ≥ 31<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = function<br>	∧ -4.label in {<newline>}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>⇒ y = ∅<br>Confidence: 0.929. Support: 119.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {function, if}<br>	∧ -4.label in {<newline>}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 199.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ -4.label not in {<newline>}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 21382.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.526315789473684, "max_conf": 0.9995791912078857, "max_support": 21382, "min_conf": 0.9234327077865601, "min_support": 91, "num_rules": 19}}
```
</details>
