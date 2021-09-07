# Model report for file:///tmp/top-repos-quality-repos-vhcdciu9/advancedreact.git HEAD 636dd6fdc1019e76ca672f40c5b10ee4b257b8cf

### Dump

```json
{'created_at': '2021-09-02 00:46:31',
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
 'size': '18.2 kB',
 'tags': [],
 'uuid': '9008963b-64f0-49ea-8043-ba7083834b7c',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-vhcdciu9/advancedreact.git 636dd6fdc1019e76ca672f40c5b10ee4b257b8cf

# javascript
52 rules, avg.len. 10.3
## train
PPCR: 0.935670
### report
macro
{'f1-score': 0.9043379917119931,
 'precision': 0.9520995654509803,
 'recall': 0.8711901424358911,
 'support': 47227}
micro
{'f1-score': 0.9687890401677007,
 'precision': 0.9687890401677007,
 'recall': 0.9687890401677007,
 'support': 47227}
weighted
{'f1-score': 0.9677312158781893,
 'precision': 0.968397584483018,
 'recall': 0.9687890401677007,
 'support': 47227}
### report_full
macro
{'f1-score': 0.8380091485583536,
 'precision': 0.9520995654509803,
 'recall': 0.7800168354414938,
 'support': 50474}
micro
{'f1-score': 0.9365922559646268,
 'precision': 0.9687890401677007,
 'recall': 0.9064666957245314,
 'support': 50474}
weighted
{'f1-score': 0.9303813189118358,
 'precision': 0.966327974485897,
 'recall': 0.9064666957245314,
 'support': 50474}
## test
PPCR: 0.933840
### report
macro
{'f1-score': 0.8977075906899791,
 'precision': 0.9422869812207979,
 'recall': 0.8649694380875207,
 'support': 11927}
micro
{'f1-score': 0.9677202984824348,
 'precision': 0.9677202984824348,
 'recall': 0.9677202984824348,
 'support': 11927}
weighted
{'f1-score': 0.9666551344453512,
 'precision': 0.9671093453952436,
 'recall': 0.9677202984824348,
 'support': 11927}
### report_full
macro
{'f1-score': 0.8277812719840312,
 'precision': 0.9422869812207979,
 'recall': 0.767049897204841,
 'support': 12772}
micro
{'f1-score': 0.9346127373577878,
 'precision': 0.9677202984824348,
 'recall': 0.9036955840901973,
 'support': 12772}
weighted
{'f1-score': 0.9289219981472565,
 'precision': 0.9646737155033059,
 'recall': 0.9036955840901973,
 'support': 12772}
```

## javascript
### Summary
38 rules, avg.len. 10.1

| | |
|-|-|
|Min support|95|
|Max support|15189|
|Min confidence|0.9272623062133789|
|Max confidence|0.9993026256561279|

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
| 1 | `  -1.label in {<space>}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.999. Support: 525.` |
| 2 | `  -1.label not in {<space>}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 420.` |
| 3 | `  -1.label not in {<space>}<br>	∧ +1.reserved = ><br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 265.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {,, >}<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 203.` |
| 5 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles in {KEY}<br>	∧ +1.reserved not in {,, >}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 136.` |
| 6 | `  •••start_col ≥ 26<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {KEY}<br>	∧ +1.reserved not in {,, >}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.936. Support: 118.` |
| 7 | `  -1.label not in {<space>}<br>	∧ -1.reserved = <<br>	∧ -1.roles not in {KEY}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 120.` |
| 8 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, <, {}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 119.` |
| 9 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, <, {}<br>	∧ -1.roles not in {KEY}<br>	∧ +1.reserved not in {), ,, >}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 4141.` |
| 10 | `  -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.999. Support: 402.` |
| 11 | `  •••start_col ≥ 4<br>	∧ -1.reserved = ;<br>	∧ -3.roles in {LITERAL}<br>	∧ +1.reserved not in {}}<br>	∧ +3.reserved = =<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎⏎<br>Confidence: 0.995. Support: 98.` |
| 12 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;}<br>	∧ -3.internal_type = JSXIdentifier<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = "<br>Confidence: 0.998. Support: 249.` |
| 13 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;}<br>	∧ -3.internal_type not in {JSXIdentifier}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.990. Support: 1223.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.roles in {MAP}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 1217.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = "<br>Confidence: 0.998. Support: 238.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.999. Support: 400.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.949. Support: 366.` |
| 18 | `  •••start_col ≥ 14<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 278.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.diff_col ≤ 10<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ +2.reserved = :<br>	∧ +3.roles in {MAP}<br>	∧ +4.reserved not in {,}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 104.` |
| 20 | `  -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 873.` |
| 21 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {BLOCK, DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 323.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {IMPORT} and not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 519.` |
| 23 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.939. Support: 448.` |
| 24 | `  -1.internal_type = Identifier<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {IF} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.932. Support: 154.` |
| 25 | `  -1.internal_type not in {CommentLine, Identifier, StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.diff_col ≥ 3<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {IF} and not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 345.` |
| 26 | `  -1.diff_col ≥ 8<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.length ≥ 5<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {MODULE} and not in {DECLARATION, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 169.` |
| 27 | `  -1.diff_col ≤ 7<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.length ≥ 5<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {MODULE} and not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 425.` |
| 28 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = return<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, MODULE}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 138.` |
| 29 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = if<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, MODULE}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 97.` |
| 30 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, return}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, IF, MODULE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 604.` |
| 31 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, MODULE}<br>	∧ ^2.internal_type = ExpressionStatement<br>⇒ y = ␣<br>Confidence: 0.997. Support: 160.` |
| 32 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, return}<br>	∧ -2.label in {<space>}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +2.reserved = {<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {DECLARATION, IF, MODULE, OPERATOR}<br>	∧ ^2.internal_type = ExpressionStatement<br>⇒ y = ∅<br>Confidence: 0.995. Support: 102.` |
| 33 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, return}<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {DECLARATION, IF, MODULE, OPERATOR}<br>	∧ ^2.internal_type = ExpressionStatement<br>⇒ y = ∅<br>Confidence: 0.965. Support: 522.` |
| 34 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, return}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {INCOMPLETE} and not in {IMPORT, MAP}<br>	∧ +2.reserved = ><br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {DECLARATION, IF, MODULE, OPERATOR}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 717.` |
| 35 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, return}<br>	∧ -2.roles not in {MAP}<br>	∧ -5.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +2.reserved not in {=, >}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {DECLARATION, IF, MODULE, OPERATOR}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>	∧ ^2.roles in {TYPE}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 95.` |
| 36 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, return}<br>	∧ -2.roles not in {MAP}<br>	∧ -2.length ≤ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +2.reserved not in {=, >}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {DECLARATION, IF, MODULE, OPERATOR}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>	∧ ^2.roles not in {TYPE}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 820.` |
| 37 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), {, }}<br>	∧ +1.roles not in {CALLEE, IMPORT, MAP}<br>	∧ +2.reserved not in {=, >}<br>	∧ +2.roles not in {COMMENT, INCOMPLETE}<br>	∧ ^1.roles not in {DECLARATION, IF, MODULE, OPERATOR}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>	∧ ^2.roles not in {TYPE}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 359.` |
| 38 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, return}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), {, }}<br>	∧ +1.roles not in {CALLEE, IMPORT, MAP}<br>	∧ +2.reserved not in {=, >}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {DECLARATION, IF, MODULE, OPERATOR}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>	∧ ^2.roles not in {TYPE}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 15189.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 10.131578947368421, "max_conf": 0.9993026256561279, "max_support": 15189, "min_conf": 0.9272623062133789, "min_support": 95, "num_rules": 38}}
```
</details>
