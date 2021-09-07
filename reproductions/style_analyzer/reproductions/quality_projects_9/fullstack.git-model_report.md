# Model report for file:///tmp/top-repos-quality-repos-vtewvg7z/fullstack.git HEAD 5083c5f741dcf3a068db10fa1a2025728435dc80

### Dump

```json
{'created_at': '2021-08-31 21:47:10',
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
 'size': '19.6 kB',
 'tags': [],
 'uuid': '19ad4b5f-10f0-4473-9426-b607b9bac500',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-vtewvg7z/fullstack.git 5083c5f741dcf3a068db10fa1a2025728435dc80

# javascript
140 rules, avg.len. 8.0
## train
PPCR: 0.970822
### report
macro
{'f1-score': 0.7794281634627436,
 'precision': 0.8829600757881075,
 'recall': 0.720739928937278,
 'support': 20562}
micro
{'f1-score': 0.8923742826573291,
 'precision': 0.8923742826573291,
 'recall': 0.8923742826573291,
 'support': 20562}
weighted
{'f1-score': 0.8878937718551004,
 'precision': 0.8920303355049811,
 'recall': 0.8923742826573291,
 'support': 20562}
### report_full
macro
{'f1-score': 0.7364847918322961,
 'precision': 0.8829600757881075,
 'recall': 0.6755576263867309,
 'support': 21180}
micro
{'f1-score': 0.8791624742465622,
 'precision': 0.8923742826573291,
 'recall': 0.8663361661945231,
 'support': 21180}
weighted
{'f1-score': 0.8681134000758564,
 'precision': 0.8921056508985262,
 'recall': 0.8663361661945231,
 'support': 21180}
## test
PPCR: 0.976235
### report
macro
{'f1-score': 0.6317866214776513,
 'precision': 0.7688541219457471,
 'recall': 0.5783477552358149,
 'support': 4190}
micro
{'f1-score': 0.8441527446300716,
 'precision': 0.8441527446300716,
 'recall': 0.8441527446300716,
 'support': 4190}
weighted
{'f1-score': 0.8299953130249137,
 'precision': 0.837239323248944,
 'recall': 0.8441527446300716,
 'support': 4190}
### report_full
macro
{'f1-score': 0.6115612294950488,
 'precision': 0.7688541219457471,
 'recall': 0.5587511913740599,
 'support': 4292}
micro
{'f1-score': 0.8340014147606696,
 'precision': 0.8441527446300716,
 'recall': 0.8240913327120224,
 'support': 4292}
weighted
{'f1-score': 0.8135970872970741,
 'precision': 0.8326337360428975,
 'recall': 0.8240913327120224,
 'support': 4292}
```

## javascript
### Summary
65 rules, avg.len. 7.9

| | |
|-|-|
|Min support|146|
|Max support|6547|
|Min confidence|0.9202004671096802|
|Max confidence|0.9989473819732666|

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
| 1 | `  +1.reserved = ><br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 201.` |
| 2 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.939. Support: 3566.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 1356.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {QUALIFIED} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 1193.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.948. Support: 453.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 835.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, =}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 146.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, =}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.941. Support: 415.` |
| 9 | `  -1.reserved not in {{}<br>	∧ +1.reserved = ><br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 195.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.920. Support: 1297.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 831.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles in {LITERAL} and not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.951. Support: 477.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, =}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 223.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.roles in {QUALIFIED} and not in {DECLARATION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.924. Support: 1299.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 795.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -4.length ≥ 11<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, QUALIFIED}<br>⇒ y = "<br>Confidence: 0.931. Support: 182.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.roles in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 414.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ +1.roles in {ARGUMENT, LITERAL, STRING}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.949. Support: 247.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {LITERAL} and not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.935. Support: 392.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = const<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 373.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, const}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 174.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, const}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 3200.` |
| 23 | `  -1.diff_col ≤ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 181.` |
| 24 | `  -1.diff_col ≤ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, const, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {BLOCK, DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 181.` |
| 25 | `  -1.reserved not in {{}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 154.` |
| 26 | `  -1.reserved not in {{}<br>	∧ -3.diff_col ≥ 5<br>	∧ -5.label not in {<space>}<br>	∧ +1.reserved not in {), >}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.924. Support: 1524.` |
| 27 | `  -1.reserved not in {{}<br>	∧ -3.diff_col ≤ 4<br>	∧ -3.label in {<space>}<br>	∧ -5.label not in {<space>}<br>	∧ +1.reserved not in {), >}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 160.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 572.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -4.length ≤ 10<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {CALL}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.961. Support: 241.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {LITERAL} and not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.946. Support: 438.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 240.` |
| 32 | `  -1.diff_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 211.` |
| 33 | `  •••start_col ≤ 14<br>	∧ -1.diff_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 658.` |
| 34 | `  -1.diff_col ≤ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 190.` |
| 35 | `  -1.diff_col ≤ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {BLOCK} and not in {DECLARATION, MODULE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 168.` |
| 36 | `  -1.diff_col ≤ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.length ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {BLOCK, DECLARATION, MODULE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 162.` |
| 37 | `  -1.diff_col ≤ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ ^1.roles not in {BLOCK, DECLARATION, MODULE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.948. Support: 3533.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {IDENTIFIER} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 1227.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.922. Support: 288.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 827.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.943. Support: 446.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {LITERAL} and not in {DECLARATION, IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.953. Support: 456.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 178.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {ARGUMENT}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.942. Support: 269.` |
| 45 | `  -1.diff_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 219.` |
| 46 | `  •••start_col ≤ 25<br>	∧ -1.diff_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 717.` |
| 47 | `  -1.diff_col ≤ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 235.` |
| 48 | `  -1.diff_col ≤ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {BLOCK, DECLARATION, MODULE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 185.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 456.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {LITERAL} and not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.949. Support: 419.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 192.` |
| 52 | `  -1.diff_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 217.` |
| 53 | `  -1.diff_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.diff_offset ≤ 11<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 649.` |
| 54 | `  -1.diff_col ≤ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 164.` |
| 55 | `  -1.diff_col ≤ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {BLOCK} and not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 160.` |
| 56 | `  -1.diff_col ≤ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.length ≥ 2<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {BLOCK, DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {FILE, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 154.` |
| 57 | `  -1.diff_col ≤ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {BLOCK, DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {FILE, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 3297.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.roles in {MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 475.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -4.length ≥ 10<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = "<br>Confidence: 0.951. Support: 153.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = const<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 377.` |
| 61 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 230.` |
| 62 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 178.` |
| 63 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, OPERATOR}<br>	∧ ^2.roles not in {FILE}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.969. Support: 177.` |
| 64 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, const, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles in {STATEMENT} and not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 357.` |
| 65 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, const, {}<br>	∧ -5.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, STATEMENT}<br>	∧ ^2.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 6547.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.907692307692308, "max_conf": 0.9989473819732666, "max_support": 6547, "min_conf": 0.9202004671096802, "min_support": 146, "num_rules": 65}}
```
</details>
