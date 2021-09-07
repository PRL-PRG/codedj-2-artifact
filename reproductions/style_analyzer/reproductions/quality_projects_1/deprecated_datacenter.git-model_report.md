# Model report for file:///tmp/top-repos-quality-repos-8l0gd03t/deprecated_datacenter.git HEAD cef6b927883722188f777c210169a9dd1285fae5

### Dump

```json
{'created_at': '2021-09-01 23:44:21',
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
 'size': '36.5 kB',
 'tags': [],
 'uuid': '9b4c2335-b419-4d6e-a0ae-d0b5108cf180',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-8l0gd03t/deprecated_datacenter.git cef6b927883722188f777c210169a9dd1285fae5

# javascript
102 rules, avg.len. 10.3
## train
PPCR: 0.871555
### report
macro
{'f1-score': 0.45357161763994297,
 'precision': 0.5049292351447682,
 'recall': 0.42801554400794317,
 'support': 265040}
micro
{'f1-score': 0.9608625113190462,
 'precision': 0.9608625113190462,
 'recall': 0.9608625113190462,
 'support': 265040}
weighted
{'f1-score': 0.9589174005387585,
 'precision': 0.9591757618844229,
 'recall': 0.9608625113190462,
 'support': 265040}
### report_full
macro
{'f1-score': 0.3130627341060296,
 'precision': 0.5049292351447682,
 'recall': 0.26126291433804244,
 'support': 304100}
micro
{'f1-score': 0.894918649190006,
 'precision': 0.9608625113190462,
 'recall': 0.8374449194343966,
 'support': 304100}
weighted
{'f1-score': 0.8718490725457778,
 'precision': 0.9489975548616102,
 'recall': 0.8374449194343966,
 'support': 304100}
## test
PPCR: 0.865894
### report
macro
{'f1-score': 0.4484487256794298,
 'precision': 0.4682221452502516,
 'recall': 0.4349482958432564,
 'support': 69740}
micro
{'f1-score': 0.9609836535704044,
 'precision': 0.9609836535704044,
 'recall': 0.9609836535704044,
 'support': 69740}
weighted
{'f1-score': 0.9589286987823504,
 'precision': 0.958338474923498,
 'recall': 0.9609836535704044,
 'support': 69740}
### report_full
macro
{'f1-score': 0.31468728562300746,
 'precision': 0.4682221452502516,
 'recall': 0.26240844443367606,
 'support': 80541}
micro
{'f1-score': 0.8919158110473048,
 'precision': 0.9609836535704044,
 'recall': 0.8321103537328813,
 'support': 80541}
weighted
{'f1-score': 0.8682874334337143,
 'precision': 0.9450093011790704,
 'recall': 0.8321103537328813,
 'support': 80541}
```

## javascript
### Summary
60 rules, avg.len. 9.9

| | |
|-|-|
|Min support|93|
|Max support|53840|
|Min confidence|0.9201388955116272|
|Max confidence|0.9994033575057983|

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
                     'min_samples_split': 181,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.958. Support: 1069.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.977. Support: 236.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 46628.` |
| 4 | `  -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.957. Support: 4755.` |
| 5 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 10476.` |
| 6 | `  •••start_col ≥ 16<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles in {FOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 101.` |
| 7 | `  -1.reserved = {<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.948. Support: 3156.` |
| 8 | `  -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 2514.` |
| 9 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>⇒ y = ∅<br>Confidence: 0.999. Support: 622.` |
| 10 | `  -1.diff_col ≥ 6<br>	∧ -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.938. Support: 443.` |
| 11 | `  •••start_line ≤ 12<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.930. Support: 1117.` |
| 12 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.diff_line ≥ 1<br>	∧ -3.roles in {STRING}<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.986. Support: 756.` |
| 13 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.diff_line = 0<br>	∧ -3.label in {<-space>}<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {CALL} and not in {IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.920. Support: 182.` |
| 14 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.diff_line = 0<br>	∧ -3.label not in {<-space>}<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.934. Support: 2683.` |
| 15 | `  •••start_col ≤ 7<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -2.label in {<-space>}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles not in {FUNCTION}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {SCOPE} and not in {IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.938. Support: 200.` |
| 16 | `  -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = [<br>	∧ +1.roles in {STRING} and not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {RIGHT}<br>⇒ y = "<br>Confidence: 0.935. Support: 130.` |
| 17 | `  -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = [<br>	∧ +1.roles not in {KEY, STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.943. Support: 185.` |
| 18 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = )<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.reserved not in {)}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.reserved not in {.}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved not in {:}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 270.` |
| 19 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.reserved not in {)}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.reserved not in {.}<br>	∧ -5.label in {<newline>}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved not in {:}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, INITIALIZATION}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 1081.` |
| 20 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.reserved not in {)}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.reserved not in {.}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved not in {:}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 23511.` |
| 21 | `  -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -4.roles in {RIGHT}<br>	∧ +1.roles not in {KEY, STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 422.` |
| 22 | `  -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 1498.` |
| 23 | `  -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.diff_offset ≤ 1<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 244.` |
| 24 | `  •••start_line ≥ 13<br>	∧ -1.internal_type = StringLiteral<br>	∧ -2.length ≤ 17<br>	∧ -5.diff_offset ≥ 17<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.945. Support: 7059.` |
| 25 | `  •••start_line ≤ 12<br>	∧ -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {MAP} and not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.963. Support: 1279.` |
| 26 | `  •••start_line ≤ 12<br>	∧ -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, MAP}<br>⇒ y = '<br>Confidence: 0.979. Support: 163.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 7519.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<-tab>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.950. Support: 919.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<-space>} and not in {<-tab>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.944. Support: 169.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<-space>} and not in {<-space>, <-tab>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.995. Support: 93.` |
| 31 | `  •••start_col ≤ 35<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<-space>, <-tab>}<br>	∧ -4.label in {<-tab>}<br>	∧ -5.diff_offset ≤ 24<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.929. Support: 233.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 1499.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -5.diff_offset ≥ 9<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.939. Support: 1078.` |
| 34 | `  •••start_line ≤ 31<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -5.diff_offset ≤ 8<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.962. Support: 93.` |
| 35 | `  -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {BLOCK} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.920. Support: 144.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {COMMENT}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {BLOCK, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 3442.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {COMMENT}<br>	∧ -2.diff_col ≥ 4<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {BLOCK, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 830.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {COMMENT}<br>	∧ -2.diff_col ≤ 3<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ -4.label not in {<space>}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {BINARY} and not in {BLOCK, IDENTIFIER, MAP}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 360.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {COMMENT}<br>	∧ -2.diff_col ≤ 3<br>	∧ -2.roles in {EXPRESSION}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ -4.label not in {<space>}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {BINARY, BLOCK, IDENTIFIER, MAP}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 259.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = BinaryExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 3342.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = (<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = BinaryExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 160.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = BinaryExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 869.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 294.` |
| 44 | `  •••start_col ≥ 20<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -2.label not in {<space>}<br>	∧ -4.diff_offset ≤ 14<br>	∧ -5.reserved = function<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {MAP}<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 369.` |
| 45 | `  •••start_col ≥ 20<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -2.label not in {<space>}<br>	∧ -2.reserved not in {'}<br>	∧ -2.roles in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 14<br>	∧ -4.label in {<space>}<br>	∧ -5.diff_offset ≤ 13<br>	∧ -5.reserved not in {=, function}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {MAP}<br>	∧ +2.length ≤ 10<br>	∧ +5.roles not in {CALL}<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 97.` |
| 46 | `  •••start_col ≤ 36<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -2.label not in {<space>}<br>	∧ -2.reserved not in {'}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 14<br>	∧ -5.diff_offset ≤ 13<br>	∧ -5.label in {<space>}<br>	∧ -5.reserved not in {=, function}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {MAP}<br>	∧ +2.length ≤ 10<br>	∧ +5.roles not in {CALL}<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 152.` |
| 47 | `  •••start_col ≥ 15<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ -4.diff_offset ≥ 14<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 511.` |
| 48 | `  •••start_col ≤ 14<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ -2.label in {<newline>}<br>	∧ -4.diff_offset ≥ 14<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 172.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ -4.diff_offset ≤ 13<br>	∧ -4.label in {<space>}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 253.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ -3.diff_line ≥ 2<br>	∧ -4.diff_offset ≤ 13<br>	∧ -4.label not in {<space>}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 105.` |
| 51 | `  •••start_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ -3.diff_line ≤ 1<br>	∧ -4.diff_offset ≤ 13<br>	∧ -4.label not in {<space>}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 752.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ -1.roles in {MAP}<br>	∧ -2.label in {<+space>}<br>	∧ -2.length ≥ 9<br>	∧ +1.reserved = :<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.935. Support: 208.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ -1.roles in {MAP}<br>	∧ -2.length ≤ 3<br>	∧ +1.reserved = :<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 499.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {:, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 226.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = =<br>	∧ +1.reserved not in {:, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 214.` |
| 56 | `  •••start_line ≤ 254<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = function<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 618.` |
| 57 | `  •••start_line ≥ 108<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = function<br>	∧ -4.label in {<space>}<br>	∧ +1.reserved not in {:, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression, CallExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 196.` |
| 58 | `  •••start_line ≥ 108<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = function<br>	∧ -4.diff_offset ≤ 13<br>	∧ -4.label not in {<space>}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression, CallExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 252.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, =, function, if}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {:, =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {ASSIGNMENT}<br>	∧ ^1.internal_type not in {BinaryExpression}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 793.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, =, function, if}<br>	∧ +1.reserved not in {:, =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {ASSIGNMENT}<br>	∧ ^1.internal_type not in {BinaryExpression}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 53840.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.9, "max_conf": 0.9994033575057983, "max_support": 53840, "min_conf": 0.9201388955116272, "min_support": 93, "num_rules": 60}}
```
</details>
