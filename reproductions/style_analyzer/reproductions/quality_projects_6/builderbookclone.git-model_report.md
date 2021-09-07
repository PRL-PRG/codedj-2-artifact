# Model report for file:///tmp/top-repos-quality-repos-asunvohv/builderbookclone.git HEAD 697b7e1211702b23156ebf17cb2f077a96593901

### Dump

```json
{'created_at': '2021-09-01 23:31:56',
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
 'size': '20.9 kB',
 'tags': [],
 'uuid': '64d3249d-ce3b-4244-a737-775de7fa31a9',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-asunvohv/builderbookclone.git 697b7e1211702b23156ebf17cb2f077a96593901

# javascript
75 rules, avg.len. 10.5
## train
PPCR: 0.956691
### report
macro
{'f1-score': 0.7394901349331428,
 'precision': 0.7510635353708265,
 'recall': 0.7299822866989342,
 'support': 118887}
micro
{'f1-score': 0.9626872576480187,
 'precision': 0.9626872576480187,
 'recall': 0.9626872576480187,
 'support': 118887}
weighted
{'f1-score': 0.9616210264998497,
 'precision': 0.9613597048085624,
 'recall': 0.9626872576480187,
 'support': 118887}
### report_full
macro
{'f1-score': 0.714088032124365,
 'precision': 0.7510635353708265,
 'recall': 0.6840711559645424,
 'support': 124269}
micro
{'f1-score': 0.9413791968941749,
 'precision': 0.9626872576480187,
 'recall': 0.9209939727526575,
 'support': 124269}
weighted
{'f1-score': 0.9388180344538862,
 'precision': 0.9599278208430452,
 'recall': 0.9209939727526575,
 'support': 124269}
## test
PPCR: 0.959579
### report
macro
{'f1-score': 0.7361440290801188,
 'precision': 0.7465347010906813,
 'recall': 0.7297684342288183,
 'support': 29176}
micro
{'f1-score': 0.9584247326569784,
 'precision': 0.9584247326569784,
 'recall': 0.9584247326569784,
 'support': 29176}
weighted
{'f1-score': 0.9572365551047375,
 'precision': 0.9576106231555134,
 'recall': 0.9584247326569784,
 'support': 29176}
### report_full
macro
{'f1-score': 0.7120426439264549,
 'precision': 0.7465347010906813,
 'recall': 0.6851465451579037,
 'support': 30405}
micro
{'f1-score': 0.9386549403333277,
 'precision': 0.9584247326569784,
 'recall': 0.9196842624568328,
 'support': 30405}
weighted
{'f1-score': 0.9359561516656653,
 'precision': 0.9560232195659821,
 'recall': 0.9196842624568328,
 'support': 30405}
```

## javascript
### Summary
53 rules, avg.len. 9.9

| | |
|-|-|
|Min support|91|
|Max support|15943|
|Min confidence|0.9256526827812195|
|Max confidence|0.9999009966850281|

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
| 1 | `  +1.length ≥ 2<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 5049.` |
| 2 | `  +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 3621.` |
| 3 | `  -1.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.996. Support: 2360.` |
| 4 | `  •••start_col ≥ 9<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ -3.roles in {PATHNAME}<br>	∧ +1.reserved not in {import}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.998. Support: 201.` |
| 5 | `  •••start_col ≥ 9<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ -3.roles not in {PATHNAME}<br>	∧ +1.length ≥ 3<br>	∧ +2.length ≥ 3<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.929. Support: 189.` |
| 6 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.984. Support: 798.` |
| 7 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 1846.` |
| 8 | `  •••start_col ≤ 35<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -2.roles not in {VALUE}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles in {STRING}<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.995. Support: 111.` |
| 9 | `  •••start_col ≥ 28<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -2.roles not in {VALUE}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles not in {STRING}<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.941. Support: 365.` |
| 10 | `  •••start_col ≤ 27<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.926. Support: 1421.` |
| 11 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -3.internal_type = JSXIdentifier<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, ObjectExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 467.` |
| 12 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -2.label in {<space>}<br>	∧ -3.internal_type not in {JSXIdentifier}<br>	∧ +1.roles in {IMPORT} and not in {VALUE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, ObjectExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 155.` |
| 13 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -2.label in {<space>}<br>	∧ -3.internal_type not in {JSXIdentifier}<br>	∧ -3.reserved not in {,}<br>	∧ +1.roles not in {IMPORT, VALUE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, ObjectExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.960. Support: 2378.` |
| 14 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, ObjectExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.979. Support: 727.` |
| 15 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, ObjectExpression}<br>	∧ ^1.roles in {STRING} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 723.` |
| 16 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {STRING}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, ObjectExpression}<br>	∧ ^1.roles not in {QUALIFIED, STRING}<br>⇒ y = "<br>Confidence: 0.981. Support: 449.` |
| 17 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.length ≥ 13<br>	∧ ^1.internal_type = File<br>⇒ y = ∅<br>Confidence: 0.966. Support: 192.` |
| 18 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = <<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, ObjectExpression}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 295.` |
| 19 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, <, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -2.label in {<-space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, JSXElement, ObjectExpression}<br>	∧ ^1.roles in {BODY} and not in {QUALIFIED, STRING}<br>⇒ y = ⏎⏎<br>Confidence: 0.948. Support: 319.` |
| 20 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, <, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -2.label in {<-space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, JSXElement, ObjectExpression}<br>	∧ ^1.roles not in {BODY, QUALIFIED, STRING}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 117.` |
| 21 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, <, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.roles in {EXPRESSION, INCOMPLETE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, ObjectExpression}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 188.` |
| 22 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, <, {}<br>	∧ -1.roles not in {INCOMPLETE, STRING}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -2.label not in {<-space>}<br>	∧ -4.diff_offset ≥ 16<br>	∧ +1.roles in {INCOMPLETE} and not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ +4.length ≥ 3<br>	∧ ^1.internal_type not in {File, JSXElement, ObjectExpression}<br>	∧ ^1.roles not in {QUALIFIED, STRING}<br>⇒ y = ⏎<br>Confidence: 0.972. Support: 126.` |
| 23 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, <, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ +1.length ≥ 2<br>	∧ +4.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type not in {File, JSXElement, ObjectExpression}<br>	∧ ^1.roles not in {QUALIFIED, STRING}<br>	∧ ^2.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 11727.` |
| 24 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles in {INCOMPLETE}<br>	∧ +1.length ≤ 1<br>⇒ y = "<br>Confidence: 0.998. Support: 287.` |
| 25 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles not in {INCOMPLETE}<br>	∧ +1.length ≤ 1<br>⇒ y = '<br>Confidence: 0.993. Support: 3501.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 2467.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -3.diff_offset ≤ 3<br>	∧ -3.label in {<space>}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 95.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {}}<br>	∧ ^1.roles in {INCOMPLETE} and not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 1308.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.length ≥ 2<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 104.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ -2.length ≤ 1<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 725.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {)}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, VARIABLE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.930. Support: 2937.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {VARIABLE}<br>	∧ ^2.internal_type = JSXOpeningElement<br>	∧ ^2.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1104.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {VARIABLE}<br>	∧ ^2.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 728.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {VARIABLE}<br>	∧ ^2.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 2767.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {IDENTIFIER}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = ><br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 206.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.diff_offset ≥ 5<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles in {OPERATOR} and not in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 881.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 444.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 286.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.997. Support: 146.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = <<br>⇒ y = ␣<br>Confidence: 0.945. Support: 247.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.reserved = return<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {CALL}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 254.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.reserved = return<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {CALL}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {<, >}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.996. Support: 129.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.reserved = :<br>	∧ +1.roles not in {CALL}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {<, >}<br>	∧ +4.length ≥ 2<br>	∧ ^1.roles in {EXPRESSION, IF}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 91.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {CALL}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles in {IF} and not in {EXPRESSION, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 1027.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {CALL}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {<, >}<br>	∧ +2.length ≥ 2<br>	∧ ^1.roles not in {IF}<br>	∧ ^2.roles in {SCOPE}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 129.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {CALL}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ +2.length ≤ 1<br>	∧ ^1.roles not in {IF, OPERATOR, VARIABLE}<br>	∧ ^2.roles in {SCOPE}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 296.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -5.roles in {ARGUMENT}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {CALL}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {IF, OPERATOR, VARIABLE}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 376.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -4.reserved not in {=}<br>	∧ -5.roles not in {ARGUMENT}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {CALL}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type = JSXAttribute<br>	∧ ^1.roles not in {IF, OPERATOR, VARIABLE}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1560.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -4.reserved not in {=}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {CALL}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {<, >}<br>	∧ ^1.internal_type not in {JSXAttribute}<br>	∧ ^1.roles not in {IF}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 93.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -4.reserved not in {=}<br>	∧ -5.roles not in {ARGUMENT}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {CALL}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {IF, OPERATOR, VARIABLE}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 15943.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -5.roles not in {ARGUMENT}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {CALL}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {IF, OPERATOR, VARIABLE}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 101.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.reserved not in {return}<br>	∧ -4.reserved not in {=}<br>	∧ -5.roles not in {ARGUMENT}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {CALL}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {IF, OPERATOR, VARIABLE}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 11929.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length = 0<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ⏎<br>Confidence: 0.940. Support: 340.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.88679245283019, "max_conf": 0.9999009966850281, "max_support": 15943, "min_conf": 0.9256526827812195, "min_support": 91, "num_rules": 53}}
```
</details>
