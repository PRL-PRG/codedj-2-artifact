# Model report for file:///tmp/top-repos-quality-repos-aqntce2w/microsoft-devops-projects.git HEAD 2d2ad0079acaee10c8a39de951209174b593caca

### Dump

```json
{'created_at': '2021-09-02 03:54:02',
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
 'size': '38.6 kB',
 'tags': [],
 'uuid': '5e3b8a8d-2586-4a65-a932-b1f8657c89d6',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-aqntce2w/microsoft-devops-projects.git 2d2ad0079acaee10c8a39de951209174b593caca

# javascript
388 rules, avg.len. 10.4
## train
PPCR: 0.905440
### report
macro
{'f1-score': 0.4882041706157268,
 'precision': 0.5202158445534164,
 'recall': 0.4697711988102492,
 'support': 86455}
micro
{'f1-score': 0.9147070730437799,
 'precision': 0.91470707304378,
 'recall': 0.91470707304378,
 'support': 86455}
weighted
{'f1-score': 0.9098545612040883,
 'precision': 0.9096452033776354,
 'recall': 0.91470707304378,
 'support': 86455}
### report_full
macro
{'f1-score': 0.38591862670063676,
 'precision': 0.5202158445534164,
 'recall': 0.3285267829909123,
 'support': 95484}
micro
{'f1-score': 0.8693133412847163,
 'precision': 0.91470707304378,
 'recall': 0.8282120564701939,
 'support': 95484}
weighted
{'f1-score': 0.850076717710467,
 'precision': 0.8968130045298313,
 'recall': 0.8282120564701939,
 'support': 95484}
## test
PPCR: 0.890613
### report
macro
{'f1-score': 0.47052200065847016,
 'precision': 0.5131693404026768,
 'recall': 0.44642861187937566,
 'support': 18132}
micro
{'f1-score': 0.8981358923450253,
 'precision': 0.8981358923450253,
 'recall': 0.8981358923450253,
 'support': 18132}
weighted
{'f1-score': 0.8927127389998284,
 'precision': 0.8925111376255376,
 'recall': 0.8981358923450253,
 'support': 18132}
### report_full
macro
{'f1-score': 0.36070900068441947,
 'precision': 0.5131693404026768,
 'recall': 0.30492536817941157,
 'support': 20359}
micro
{'f1-score': 0.8461718323763997,
 'precision': 0.8981358923450253,
 'recall': 0.7998919396826956,
 'support': 20359}
weighted
{'f1-score': 0.8232726125735242,
 'precision': 0.8774019877681833,
 'recall': 0.7998919396826956,
 'support': 20359}
```

## javascript
### Summary
213 rules, avg.len. 10.8

| | |
|-|-|
|Min support|136|
|Max support|12288|
|Min confidence|0.9212523698806763|
|Max confidence|0.999834418296814|

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
| 1 | `  •••start_line ≤ 112<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +2.reserved = .<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 613.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +2.reserved not in {.}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 10669.` |
| 3 | `  •••start_line ≥ 126<br>	∧ -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles in {MAP}<br>⇒ y = '<br>Confidence: 0.949. Support: 424.` |
| 4 | `  •••start_line ≤ 125<br>	∧ -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.929. Support: 3261.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -3.roles in {MAP}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.969. Support: 1747.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -2.reserved = (<br>	∧ -3.roles not in {MAP}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.997. Support: 192.` |
| 7 | `  •••start_line ≤ 91<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 1335.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type = ObjectProperty<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.987. Support: 348.` |
| 9 | `  -1.diff_col ≥ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.969. Support: 465.` |
| 10 | `  -1.diff_col ≥ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≥ 12<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.997. Support: 507.` |
| 11 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.diff_col ≥ 5<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 18<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 256.` |
| 12 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.diff_col ≥ 5<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 17<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 5699.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.length ≤ 2<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 163.` |
| 14 | `  •••start_col ≥ 14<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {(, )}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 3107.` |
| 15 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 427.` |
| 16 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -4.diff_line = 0<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 946.` |
| 17 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {MAP} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.973. Support: 275.` |
| 18 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 288.` |
| 19 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ ^1.roles in {IF} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 410.` |
| 20 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, =, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -3.diff_offset ≤ 3<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ ^1.roles in {FOR} and not in {IF, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.928. Support: 159.` |
| 21 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, =, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.length ≤ 16<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved = (<br>	∧ ^1.roles not in {FOR, IF, OPERATOR, QUALIFIED, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 268.` |
| 22 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, =, {}<br>	∧ -2.label in {<space>}<br>	∧ -2.length ≤ 16<br>	∧ -3.reserved = (<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ ^1.roles not in {IF, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 147.` |
| 23 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, =, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ -2.length ≤ 16<br>	∧ +1.reserved not in {), =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ ^1.roles not in {FOR, IF, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.937. Support: 3338.` |
| 24 | `  •••start_col ≥ 17<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, =, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -2.length ≤ 16<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ ^1.roles not in {FOR, IF, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 7625.` |
| 25 | `  •••start_col ≤ 16<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, :, ;}<br>	∧ -2.diff_col ≤ 13<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -2.length ≤ 16<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ ^1.roles not in {FOR, IF, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 541.` |
| 26 | `  •••start_col ≤ 16<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, :, ;, =, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -2.length ≤ 16<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ ^1.roles not in {FOR, IF, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 4830.` |
| 27 | `  -1.roles in {STRING}<br>	∧ -4.label in {<space>}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = '<br>Confidence: 0.934. Support: 174.` |
| 28 | `  -1.roles not in {STRING}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.962. Support: 11827.` |
| 29 | `  •••start_line ≥ 126<br>	∧ -1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles in {MAP}<br>⇒ y = '<br>Confidence: 0.957. Support: 405.` |
| 30 | `  •••start_line ≤ 125<br>	∧ -1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.921. Support: 3280.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -3.roles in {MAP}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.974. Support: 1783.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -2.reserved = (<br>	∧ -3.roles not in {MAP}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = "<br>Confidence: 0.997. Support: 185.` |
| 33 | `  •••start_line ≥ 92<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -5.length ≥ 12<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 203.` |
| 34 | `  •••start_line ≤ 91<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 1391.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles in {INITIALIZATION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.974. Support: 329.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.977. Support: 502.` |
| 37 | `  -1.diff_col ≥ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.995. Support: 459.` |
| 38 | `  -1.diff_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -4.diff_offset ≥ 12<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.932. Support: 702.` |
| 39 | `  -1.diff_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +3.roles in {LITERAL}<br>	∧ ^1.internal_type = VariableDeclaration<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.983. Support: 146.` |
| 40 | `  -1.diff_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -4.roles in {MAP}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = '<br>Confidence: 0.933. Support: 142.` |
| 41 | `  -1.diff_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -4.diff_offset ≥ 6<br>	∧ -4.roles not in {MAP}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 6734.` |
| 42 | `  -1.diff_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 5<br>	∧ -4.roles not in {MAP}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 1050.` |
| 43 | `  -1.diff_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 5<br>	∧ -4.roles not in {MAP}<br>	∧ +1.roles in {BINARY, EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclaration}<br>	∧ ^1.roles in {EXPRESSION} and not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.928. Support: 200.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.937. Support: 167.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.length ≤ 2<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 193.` |
| 46 | `  •••start_col ≥ 14<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {(}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +3.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 3028.` |
| 47 | `  •••start_line ≤ 91<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -5.label in {'}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.927. Support: 144.` |
| 48 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 437.` |
| 49 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -3.diff_line = 0<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 900.` |
| 50 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {MAP} and not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.979. Support: 261.` |
| 51 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -3.length ≥ 3<br>	∧ +1.reserved = function<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 340.` |
| 52 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {IF} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 397.` |
| 53 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {IF, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 158.` |
| 54 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ -2.length ≤ 16<br>	∧ -3.internal_type not in {Identifier}<br>	∧ -3.reserved not in {(}<br>	∧ +1.reserved not in {=, function, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FOR, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.928. Support: 3558.` |
| 55 | `  •••start_col ≥ 17<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -2.length ≤ 16<br>	∧ +1.reserved not in {=, function, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FOR, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 7887.` |
| 56 | `  •••start_col ≤ 16<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, :, ;}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -2.length ≤ 16<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FOR, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 415.` |
| 57 | `  •••start_col ≤ 16<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, :, ;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -2.length ≤ 2<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FOR, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 170.` |
| 58 | `  •••start_col ≤ 16<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, :, ;, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -2.length ≤ 16<br>	∧ +1.reserved not in {(, =, function, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FOR, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 4853.` |
| 59 | `  •••start_line ≤ 254<br>	∧ -1.roles not in {STRING}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +2.reserved = .<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 797.` |
| 60 | `  -1.reserved = .<br>	∧ -1.roles not in {STRING}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +2.reserved not in {.}<br>	∧ +4.internal_type = Identifier<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 499.` |
| 61 | `  -1.roles not in {STRING}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {.}<br>	∧ +4.internal_type = Identifier<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 1980.` |
| 62 | `  -1.roles not in {STRING}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +2.reserved not in {.}<br>	∧ +4.internal_type not in {Identifier}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 7917.` |
| 63 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles in {MAP}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.975. Support: 2023.` |
| 64 | `  •••start_line ≥ 92<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -5.diff_offset ≤ 23<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles in {BOOLEAN}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 147.` |
| 65 | `  •••start_line ≤ 91<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 1317.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles in {INITIALIZATION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.987. Support: 348.` |
| 67 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.972. Support: 520.` |
| 68 | `  -1.diff_col ≥ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.992. Support: 427.` |
| 69 | `  -1.diff_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 6889.` |
| 70 | `  -1.diff_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.diff_offset ≤ 3<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 887.` |
| 71 | `  •••start_line ≤ 92<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -5.label in {'}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.934. Support: 188.` |
| 72 | `  -1.internal_type = CommentLine<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.length ≤ 93<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.955. Support: 1219.` |
| 73 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {)}<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.941. Support: 179.` |
| 74 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -2.label not in {<newline>}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 903.` |
| 75 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -3.length ≥ 3<br>	∧ +1.reserved = function<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 335.` |
| 76 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ ^1.roles in {IF} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 372.` |
| 77 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ ^1.roles not in {IF, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 147.` |
| 78 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, function, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved = (<br>	∧ ^1.roles not in {FOR, IF, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 156.` |
| 79 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, {}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.length ≤ 20<br>	∧ -3.reserved not in {(}<br>	∧ +1.reserved not in {=, function, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(, ,}<br>	∧ ^1.roles not in {FOR, IF, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 7603.` |
| 80 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.length ≤ 20<br>	∧ +1.reserved not in {(, =, function, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(, ,}<br>	∧ ^1.roles not in {FOR, IF, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.928. Support: 8895.` |
| 81 | `  •••start_line ≤ 91<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 1293.` |
| 82 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.internal_type = ObjectExpression<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.959. Support: 308.` |
| 83 | `  -1.diff_col ≥ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {COMMENT}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.995. Support: 469.` |
| 84 | `  -1.diff_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.reserved = :<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = '<br>Confidence: 0.996. Support: 142.` |
| 85 | `  -1.diff_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.reserved not in {:}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 6921.` |
| 86 | `  -1.diff_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.reserved not in {:}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 994.` |
| 87 | `  -1.diff_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.reserved = )<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclaration}<br>	∧ ^1.roles in {EXPRESSION} and not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 173.` |
| 88 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 200.` |
| 89 | `  •••start_col ≥ 14<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {(, )}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.948. Support: 3025.` |
| 90 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.label not in {<newline>}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 927.` |
| 91 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -2.length ≤ 16<br>	∧ +1.reserved not in {=, function, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ +3.reserved not in {.}<br>	∧ +4.roles not in {BINARY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FOR, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 12288.` |
| 92 | `  •••start_line ≤ 155<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +2.reserved = .<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.986. Support: 617.` |
| 93 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +2.reserved not in {.}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.979. Support: 10503.` |
| 94 | `  -1.internal_type = StringLiteral<br>	∧ -3.reserved = (<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = "<br>Confidence: 0.923. Support: 149.` |
| 95 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -2.reserved = (<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = "<br>Confidence: 0.998. Support: 207.` |
| 96 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -2.reserved not in {(}<br>	∧ -3.roles in {MAP}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.969. Support: 1684.` |
| 97 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -3.reserved = :<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.991. Support: 291.` |
| 98 | `  -1.diff_col ≥ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.963. Support: 419.` |
| 99 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.diff_col ≥ 5<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 18<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 257.` |
| 100 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.diff_col ≥ 5<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 17<br>	∧ +4.roles not in {BINARY}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 5620.` |
| 101 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.diff_col ≤ 4<br>	∧ -3.label not in {<space>}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 17<br>	∧ +4.roles not in {BINARY}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 1129.` |
| 102 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -3.diff_col ≤ 4<br>	∧ -3.label not in {<space>}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 17<br>	∧ +4.roles not in {BINARY}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 628.` |
| 103 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.diff_offset ≤ 2<br>	∧ -3.diff_col ≤ 4<br>	∧ -3.label not in {<space>}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ +1.roles in {BINARY, EXPRESSION}<br>	∧ +1.length ≤ 17<br>	∧ +4.roles not in {BINARY}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 217.` |
| 104 | `  •••start_col ≥ 14<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {(}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 3259.` |
| 105 | `  -1.internal_type = CommentLine<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.length ≤ 93<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.944. Support: 1240.` |
| 106 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {)}<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 194.` |
| 107 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, function, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles in {CALL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FOR, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 3268.` |
| 108 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {CALL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FOR, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 1130.` |
| 109 | `  •••start_line ≥ 153<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, =, function, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {CALL}<br>	∧ +2.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FOR, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 1728.` |
| 110 | `  •••start_line ≥ 153<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved not in {=, function, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved = }<br>	∧ +2.roles not in {CALL}<br>	∧ +2.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FOR, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 212.` |
| 111 | `  •••start_line ≤ 152<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, =, function, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {CALL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FOR, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 9349.` |
| 112 | `  •••start_line ≤ 114<br>	∧ -1.roles not in {STRING}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +2.reserved = .<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 585.` |
| 113 | `  -1.reserved = .<br>	∧ -1.roles not in {STRING}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +2.reserved not in {.}<br>	∧ +4.roles in {IDENTIFIER}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 449.` |
| 114 | `  -1.roles not in {STRING}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {.}<br>	∧ +4.roles in {IDENTIFIER}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 2005.` |
| 115 | `  -1.roles not in {STRING}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +2.reserved not in {.}<br>	∧ +4.roles not in {IDENTIFIER}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 7957.` |
| 116 | `  •••start_line ≥ 126<br>	∧ -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {MAP}<br>⇒ y = '<br>Confidence: 0.934. Support: 447.` |
| 117 | `  •••start_line ≤ 125<br>	∧ -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.930. Support: 3196.` |
| 118 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -3.roles in {MAP}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.977. Support: 1692.` |
| 119 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -2.reserved = (<br>	∧ -3.roles not in {MAP}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.997. Support: 181.` |
| 120 | `  •••start_line ≥ 92<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -5.diff_offset ≤ 23<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {BOOLEAN}<br>⇒ y = ␣<br>Confidence: 0.949. Support: 186.` |
| 121 | `  •••start_line ≤ 91<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 1440.` |
| 122 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {INITIALIZATION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.984. Support: 340.` |
| 123 | `  -1.diff_col ≥ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.971. Support: 427.` |
| 124 | `  -1.diff_col ≥ 9<br>	∧ -1.internal_type = CommentLine<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.997. Support: 495.` |
| 125 | `  •••start_col ≤ 65<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 2<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 490.` |
| 126 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -4.roles in {MAP}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = '<br>Confidence: 0.943. Support: 148.` |
| 127 | `  -1.diff_col ≤ 8<br>	∧ -1.diff_offset ≤ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -4.roles not in {MAP}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 6360.` |
| 128 | `  -1.diff_col ≤ 8<br>	∧ -1.diff_offset ≤ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.roles in {EXPRESSION}<br>	∧ -4.roles not in {MAP}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 1145.` |
| 129 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 164.` |
| 130 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 182.` |
| 131 | `  •••start_col ≥ 14<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {(, )}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 3084.` |
| 132 | `  -1.internal_type = CommentLine<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.length ≤ 93<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.940. Support: 1267.` |
| 133 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 392.` |
| 134 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -3.diff_line = 0<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 955.` |
| 135 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -4.diff_col ≥ 5<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type = CommentLine<br>	∧ +4.length ≥ 36<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎⏎<br>Confidence: 0.970. Support: 184.` |
| 136 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -3.length ≥ 3<br>	∧ +1.reserved = function<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 326.` |
| 137 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ ^1.roles in {IF} and not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 387.` |
| 138 | `  •••start_line ≥ 191<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -2.diff_col ≤ 2<br>	∧ +1.reserved not in {;, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 3<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ ^1.roles in {IF} and not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 169.` |
| 139 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ ^1.roles not in {IDENTIFIER, IF, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 157.` |
| 140 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, function, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved = (<br>	∧ ^1.roles not in {FOR, IDENTIFIER, IF, OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 151.` |
| 141 | `  -1.internal_type = Identifier<br>	∧ -1.reserved not in {:, ;}<br>	∧ -2.length ≥ 13<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ ^1.roles not in {FOR, IDENTIFIER, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 291.` |
| 142 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.length ≤ 12<br>	∧ -3.reserved = (<br>	∧ +1.reserved not in {), =, function, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ ^1.roles not in {FOR, IDENTIFIER, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 516.` |
| 143 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.length ≤ 12<br>	∧ -3.reserved not in {(}<br>	∧ +1.reserved not in {=, function, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ ^1.roles not in {FOR, IDENTIFIER, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 8248.` |
| 144 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.length ≤ 12<br>	∧ -3.reserved not in {(}<br>	∧ +1.reserved not in {(, =, function, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ ^1.roles not in {FOR, IDENTIFIER, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 6980.` |
| 145 | `  •••start_line ≤ 155<br>	∧ -1.roles not in {STRING}<br>	∧ +2.reserved = .<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 660.` |
| 146 | `  -1.roles not in {STRING}<br>	∧ +2.reserved not in {.}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 10760.` |
| 147 | `  •••start_line ≥ 110<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -5.diff_offset ≤ 23<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {BOOLEAN}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 152.` |
| 148 | `  •••start_line ≤ 109<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 1498.` |
| 149 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {LITERAL}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.975. Support: 302.` |
| 150 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.968. Support: 574.` |
| 151 | `  -1.diff_col ≥ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.997. Support: 458.` |
| 152 | `  -1.diff_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.reserved = :<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = '<br>Confidence: 0.989. Support: 140.` |
| 153 | `  -1.diff_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.reserved not in {:}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 6418.` |
| 154 | `  -1.diff_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.reserved not in {:}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 1137.` |
| 155 | `  •••start_line ≤ 91<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -5.reserved = '<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.924. Support: 139.` |
| 156 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -3.diff_line = 0<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 905.` |
| 157 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {MAP} and not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.973. Support: 241.` |
| 158 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved = (<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 153.` |
| 159 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.reserved = function<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved = (<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 424.` |
| 160 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ ^1.roles in {IF} and not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 372.` |
| 161 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ ^1.roles not in {IDENTIFIER, IF, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 151.` |
| 162 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles in {CALL}<br>	∧ ^1.roles not in {FOR, IDENTIFIER, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 3191.` |
| 163 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {CALL}<br>	∧ ^1.roles not in {FOR, IDENTIFIER, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 1085.` |
| 164 | `  •••start_line ≥ 89<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {CALL}<br>	∧ +2.length ≥ 2<br>	∧ ^1.roles not in {FOR, IDENTIFIER, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.948. Support: 2970.` |
| 165 | `  •••start_line ≥ 89<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved = }<br>	∧ +2.roles not in {CALL}<br>	∧ +2.length ≤ 1<br>	∧ ^1.roles not in {FOR, IDENTIFIER, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 303.` |
| 166 | `  •••start_col ≥ 10<br>	∧ •••start_line ≥ 89<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {CALL}<br>	∧ +2.length ≤ 1<br>	∧ ^1.roles not in {FOR, IDENTIFIER, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 1423.` |
| 167 | `  •••start_line ≤ 88<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {CALL}<br>	∧ ^1.roles not in {FOR, IDENTIFIER, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 7426.` |
| 168 | `  -1.roles in {STRING}<br>	∧ -3.roles in {CALL}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.934. Support: 188.` |
| 169 | `  •••start_line ≤ 155<br>	∧ -1.roles not in {STRING}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +2.reserved = .<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 636.` |
| 170 | `  -1.roles not in {STRING}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +2.reserved not in {.}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 10556.` |
| 171 | `  •••start_line ≥ 92<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -5.length ≥ 12<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.932. Support: 199.` |
| 172 | `  •••start_line ≤ 91<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 1403.` |
| 173 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type = ObjectProperty<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.958. Support: 322.` |
| 174 | `  -1.diff_col ≥ 12<br>	∧ -1.diff_offset ≥ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 435.` |
| 175 | `  •••start_col ≥ 44<br>	∧ -1.diff_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -5.diff_offset ≥ 22<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.939. Support: 140.` |
| 176 | `  •••start_col ≤ 43<br>	∧ -1.diff_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -5.diff_offset ≥ 9<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.968. Support: 607.` |
| 177 | `  -1.diff_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +3.roles in {LITERAL}<br>	∧ ^1.internal_type = VariableDeclaration<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.996. Support: 136.` |
| 178 | `  -1.diff_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -4.roles in {KEY}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = '<br>Confidence: 0.973. Support: 169.` |
| 179 | `  -1.diff_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -4.roles not in {KEY}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 6431.` |
| 180 | `  -1.diff_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.roles in {EXPRESSION}<br>	∧ -4.roles not in {KEY}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 1157.` |
| 181 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.970. Support: 280.` |
| 182 | `  •••start_col ≤ 52<br>	∧ •••start_line ≥ 89<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, {}<br>	∧ -2.label in {<space>}<br>	∧ -2.length ≤ 16<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ ^1.roles not in {IDENTIFIER, IF, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.934. Support: 265.` |
| 183 | `  •••start_line ≤ 88<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ -2.length ≤ 16<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ ^1.roles not in {FOR, IDENTIFIER, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.948. Support: 320.` |
| 184 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -2.length ≤ 16<br>	∧ -3.length ≥ 46<br>	∧ +1.reserved not in {=, function, {, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ +4.roles not in {BINARY}<br>	∧ ^1.roles not in {FOR, IDENTIFIER, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 256.` |
| 185 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -2.length ≤ 16<br>	∧ -3.length ≤ 45<br>	∧ +1.reserved not in {=, function, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ +4.roles not in {BINARY}<br>	∧ ^1.roles in {BODY} and not in {FOR, IDENTIFIER, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 1833.` |
| 186 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -2.length ≤ 16<br>	∧ -3.length ≤ 45<br>	∧ +1.reserved not in {=, function, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ +4.roles not in {BINARY}<br>	∧ ^1.roles not in {BODY, FOR, IDENTIFIER, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 10867.` |
| 187 | `  -1.reserved = .<br>	∧ -1.roles not in {STRING}<br>	∧ +2.reserved = .<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 543.` |
| 188 | `  -1.roles not in {STRING}<br>	∧ +2.reserved not in {.}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 10726.` |
| 189 | `  •••start_col ≤ 24<br>	∧ •••start_line ≤ 254<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {EXPRESSION, STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.921. Support: 527.` |
| 190 | `  •••start_line ≥ 92<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -5.length ≥ 12<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 194.` |
| 191 | `  -1.diff_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +3.roles in {LITERAL}<br>	∧ ^1.internal_type = VariableDeclaration<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.990. Support: 150.` |
| 192 | `  -1.diff_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.diff_offset ≤ 3<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 17<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 860.` |
| 193 | `  -1.internal_type = CommentLine<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.length ≤ 93<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.951. Support: 1263.` |
| 194 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 168.` |
| 195 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, function, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +4.roles in {BINARY}<br>	∧ ^1.roles not in {FOR, IF, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 181.` |
| 196 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +4.roles not in {BINARY}<br>	∧ ^1.roles not in {FOR, IF, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 3205.` |
| 197 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -3.reserved = (<br>	∧ +1.reserved not in {), ;, =, function, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +4.roles not in {BINARY}<br>	∧ ^1.roles not in {FOR, IF, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 153.` |
| 198 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -3.reserved not in {(}<br>	∧ +1.reserved not in {;, =, function, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +4.roles not in {BINARY}<br>	∧ ^1.roles not in {FOR, IF, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 7761.` |
| 199 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 11791.` |
| 200 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.internal_type = ObjectExpression<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.992. Support: 302.` |
| 201 | `  -1.diff_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.reserved = :<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.983. Support: 150.` |
| 202 | `  -1.diff_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {:}<br>	∧ -3.roles in {IDENTIFIER}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +4.roles not in {BINARY}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 3020.` |
| 203 | `  -1.diff_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.diff_col ≥ 3<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {:}<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +4.roles not in {BINARY}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 3118.` |
| 204 | `  -1.diff_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.diff_col ≤ 2<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {:}<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +4.roles not in {BINARY}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>	∧ ^2.roles in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 740.` |
| 205 | `  -1.diff_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = ,<br>	∧ -2.diff_col ≤ 2<br>	∧ -3.reserved not in {:}<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ -5.diff_col ≤ 21<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +4.roles not in {BINARY}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 439.` |
| 206 | `  -1.diff_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -2.diff_col ≤ 2<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {:}<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ -5.diff_col ≤ 21<br>	∧ +1.roles in {BINARY, EXPRESSION}<br>	∧ +4.roles not in {BINARY}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 197.` |
| 207 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 184.` |
| 208 | `  -1.internal_type = CommentLine<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.length ≤ 88<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.955. Support: 1308.` |
| 209 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -3.diff_line = 0<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.949. Support: 905.` |
| 210 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -4.diff_col ≥ 5<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type = CommentLine<br>	∧ +4.length ≥ 36<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.947. Support: 198.` |
| 211 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -3.reserved = (<br>	∧ +1.reserved not in {), =, function, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(, ,}<br>	∧ +4.roles not in {BINARY}<br>	∧ ^1.roles not in {FOR, IF, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 544.` |
| 212 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -3.reserved not in {(}<br>	∧ +1.reserved not in {=, function, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(, ,}<br>	∧ +4.roles not in {BINARY}<br>	∧ ^1.roles not in {FOR, IF, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 8241.` |
| 213 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -3.reserved not in {(}<br>	∧ +1.reserved not in {(, =, function, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(, ,}<br>	∧ +4.roles not in {BINARY}<br>	∧ ^1.roles not in {FOR, IF, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 7429.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 10.821596244131456, "max_conf": 0.999834418296814, "max_support": 12288, "min_conf": 0.9212523698806763, "min_support": 136, "num_rules": 213}}
```
</details>
