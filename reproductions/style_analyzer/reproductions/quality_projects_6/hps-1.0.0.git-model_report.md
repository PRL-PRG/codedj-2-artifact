# Model report for file:///tmp/top-repos-quality-repos-473ujmdh/hps-1.0.0.git HEAD 019360dca032e4c80e747a88ad5f69108eb026c6

### Dump

```json
{'created_at': '2021-09-02 06:02:22',
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
 'size': '22.3 kB',
 'tags': [],
 'uuid': 'e3d90979-6dc6-44c0-9327-c085aa6198b5',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-473ujmdh/hps-1.0.0.git 019360dca032e4c80e747a88ad5f69108eb026c6

# javascript
197 rules, avg.len. 8.3
## train
PPCR: 0.967742
### report
macro
{'f1-score': 0.5650542837326744,
 'precision': 0.6085415449414362,
 'recall': 0.5418974507477352,
 'support': 37350}
micro
{'f1-score': 0.9113520749665328,
 'precision': 0.9113520749665328,
 'recall': 0.9113520749665328,
 'support': 37350}
weighted
{'f1-score': 0.8982822732214107,
 'precision': 0.8896779526304446,
 'recall': 0.9113520749665328,
 'support': 37350}
### report_full
macro
{'f1-score': 0.5469900351807249,
 'precision': 0.6085415449414362,
 'recall': 0.5153428452875207,
 'support': 38595}
micro
{'f1-score': 0.8964118770162618,
 'precision': 0.9113520749665328,
 'recall': 0.8819536209353543,
 'support': 38595}
weighted
{'f1-score': 0.8768249156228016,
 'precision': 0.8786612275829655,
 'recall': 0.8819536209353543,
 'support': 38595}
## test
PPCR: 0.964504
### report
macro
{'f1-score': 0.5556261833402545,
 'precision': 0.6431836000530383,
 'recall': 0.5237599259243348,
 'support': 6630}
micro
{'f1-score': 0.9001508295625943,
 'precision': 0.9001508295625943,
 'recall': 0.9001508295625943,
 'support': 6630}
weighted
{'f1-score': 0.8882516754778489,
 'precision': 0.8925562225795035,
 'recall': 0.9001508295625943,
 'support': 6630}
### report_full
macro
{'f1-score': 0.5354205116087506,
 'precision': 0.6431836000530383,
 'recall': 0.4992794934670968,
 'support': 6874}
micro
{'f1-score': 0.8838862559241707,
 'precision': 0.9001508295625943,
 'recall': 0.8681990107652022,
 'support': 6874}
weighted
{'f1-score': 0.864926055743068,
 'precision': 0.8862414986885194,
 'recall': 0.8681990107652022,
 'support': 6874}
```

## javascript
### Summary
111 rules, avg.len. 8.1

| | |
|-|-|
|Min support|139|
|Max support|13348|
|Min confidence|0.920616090297699|
|Max confidence|0.9992877244949341|

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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.978. Support: 3504.` |
| 2 | `  -1.roles not in {STRING}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.989. Support: 3855.` |
| 3 | `  -1.roles in {IDENTIFIER}<br>	∧ +3.roles in {STRING}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 631.` |
| 4 | `  -1.roles in {IDENTIFIER}<br>	∧ +3.roles not in {STRING}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 367.` |
| 5 | `  -1.roles in {IDENTIFIER}<br>	∧ +3.roles not in {STRING}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles in {OPERATOR} and not in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 398.` |
| 6 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +3.roles not in {STRING}<br>	∧ ^1.internal_type not in {ArrowFunctionExpression, JSXElement, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 4233.` |
| 7 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = '<br>Confidence: 0.925. Support: 206.` |
| 8 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 1426.` |
| 9 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^2.internal_type = JSXElement<br>⇒ y = "<br>Confidence: 0.971. Support: 222.` |
| 10 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^2.internal_type not in {JSXElement}<br>⇒ y = '<br>Confidence: 0.934. Support: 731.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 807.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 636.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^2.internal_type = JSXOpeningElement<br>⇒ y = ∅<br>Confidence: 0.934. Support: 356.` |
| 14 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ,<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 228.` |
| 15 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ><br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 211.` |
| 16 | `  •••start_col ≥ 4<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -3.diff_col ≤ 5<br>	∧ +1.reserved not in {), ,, ;, >}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression, ObjectExpression}<br>	∧ ^1.roles in {DECLARATION}<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = ␣<br>Confidence: 0.949. Support: 384.` |
| 17 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 3560.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 3896.` |
| 19 | `  -1.roles in {IDENTIFIER}<br>	∧ +3.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 612.` |
| 20 | `  -1.roles in {IDENTIFIER}<br>	∧ +3.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {VARIABLE} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 363.` |
| 21 | `  -1.roles in {IDENTIFIER}<br>	∧ +3.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 380.` |
| 22 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +3.internal_type not in {StringLiteral}<br>	∧ +4.roles not in {LITERAL}<br>	∧ ^1.internal_type = FunctionDeclaration<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED, VARIABLE}<br>	∧ ^2.roles in {MODULE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 150.` |
| 23 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +3.internal_type not in {StringLiteral}<br>	∧ +4.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {ArrowFunctionExpression, JSXElement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED, VARIABLE}<br>	∧ ^2.roles not in {MODULE}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 3626.` |
| 24 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.948. Support: 1388.` |
| 25 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.975. Support: 217.` |
| 26 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles not in {INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.939. Support: 715.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 859.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 645.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^2.internal_type = JSXOpeningElement<br>⇒ y = ∅<br>Confidence: 0.931. Support: 370.` |
| 30 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ,<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 218.` |
| 31 | `  •••start_col ≥ 3<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -3.diff_offset ≤ 4<br>	∧ +1.reserved not in {), ,, ;, >}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {DECLARATION} and not in {QUALIFIED}<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = ␣<br>Confidence: 0.934. Support: 327.` |
| 32 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type = JSXIdentifier<br>⇒ y = "<br>Confidence: 0.923. Support: 576.` |
| 33 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type not in {JSXIdentifier}<br>⇒ y = '<br>Confidence: 0.931. Support: 821.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 356.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 224.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = ><br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.roles not in {FUNCTION}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 174.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.reserved not in {), >}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.roles not in {FUNCTION}<br>	∧ ^1.roles in {DECLARATION}<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ␣<br>Confidence: 0.997. Support: 165.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 1047.` |
| 39 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {OPERATOR} and not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 811.` |
| 40 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type = Program<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 186.` |
| 41 | `  •••start_col ≤ 9<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, }}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type = Program<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 521.` |
| 42 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {Program}<br>	∧ ^1.roles in {STATEMENT} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 552.` |
| 43 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression, Program}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.923. Support: 253.` |
| 44 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;}<br>	∧ -2.label not in {<-space>}<br>	∧ -4.diff_offset ≥ 2<br>	∧ +1.internal_type = JSXIdentifier<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved = ><br>	∧ ^1.internal_type not in {ConditionalExpression, Program, SwitchCase}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 480.` |
| 45 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;}<br>	∧ -2.label not in {<-space>}<br>	∧ -4.diff_offset ≥ 2<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {=, >}<br>	∧ ^1.internal_type not in {ConditionalExpression, Program, SwitchCase}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 12553.` |
| 46 | `  -1.internal_type = StringLiteral<br>	∧ -2.reserved not in {=}<br>⇒ y = '<br>Confidence: 0.927. Support: 776.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = '<br>Confidence: 0.936. Support: 573.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 311.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles in {DECLARATION}<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ␣<br>Confidence: 0.997. Support: 143.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved not in {(, )}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {{}<br>	∧ +2.roles in {IDENTIFIER} and not in {ARGUMENT}<br>	∧ ^1.roles in {DECLARATION}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 365.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {{}<br>	∧ +2.roles in {IDENTIFIER} and not in {ARGUMENT}<br>	∧ ^1.roles in {DECLARATION}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 175.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved not in {(, )}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {{}<br>	∧ +2.roles not in {ARGUMENT, IDENTIFIER}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 1540.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 1053.` |
| 54 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -4.diff_offset ≥ 7<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {OPERATOR} and not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 797.` |
| 55 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {MODULE} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 187.` |
| 56 | `  •••start_col ≤ 9<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, }}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {MODULE} and not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 540.` |
| 57 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, MODULE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 232.` |
| 58 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, MODULE}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 159.` |
| 59 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, return}<br>	∧ -3.reserved = ;<br>	∧ -4.diff_offset ≥ 2<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL, MAP, NAME}<br>	∧ +2.reserved not in {=}<br>	∧ +4.roles not in {FUNCTION}<br>	∧ ^1.internal_type not in {BlockStatement, ConditionalExpression}<br>	∧ ^1.roles not in {DECLARATION, MODULE, OPERATOR}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ∅<br>Confidence: 0.923. Support: 320.` |
| 60 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = /<br>	∧ -4.diff_offset ≥ 2<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL, MAP, NAME}<br>	∧ +2.reserved = ><br>	∧ +4.roles not in {FUNCTION}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {DECLARATION, MODULE, OPERATOR}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 434.` |
| 61 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, return}<br>	∧ -3.reserved not in {;}<br>	∧ -4.diff_offset ≥ 2<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL, MAP, NAME}<br>	∧ +2.reserved not in {=, >}<br>	∧ +4.roles not in {FUNCTION}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {DECLARATION, MODULE, OPERATOR}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 13348.` |
| 62 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -4.diff_offset ≥ 7<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {OPERATOR} and not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.928. Support: 857.` |
| 63 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles in {FILE}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 223.` |
| 64 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.923. Support: 240.` |
| 65 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles not in {FILE}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 156.` |
| 66 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = /<br>	∧ -4.diff_offset ≥ 2<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL, MAP, NAME}<br>	∧ +2.reserved = ><br>	∧ +4.roles not in {FUNCTION}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {FILE, SCOPE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 438.` |
| 67 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, return, {}<br>	∧ -3.reserved not in {;}<br>	∧ -4.diff_offset ≥ 2<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL, MAP, NAME}<br>	∧ +2.reserved not in {=, >}<br>	∧ +4.roles not in {FUNCTION}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {FILE, SCOPE}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 13042.` |
| 68 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.reserved not in {), >}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.roles not in {FUNCTION}<br>	∧ ^1.roles in {DECLARATION}<br>	∧ ^2.roles in {SCOPE}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 145.` |
| 69 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.internal_type = File<br>⇒ y = ∅<br>Confidence: 0.961. Support: 192.` |
| 70 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.internal_type not in {File}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 142.` |
| 71 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, return}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {IF} and not in {DECLARATION, OPERATOR}<br>	∧ ^2.internal_type not in {File}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 186.` |
| 72 | `  •••start_line ≥ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = /<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL, MAP, NAME}<br>	∧ +2.reserved = ><br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR}<br>	∧ ^2.internal_type not in {File}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 400.` |
| 73 | `  •••start_line ≥ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, return, }}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {LITERAL, MAP, NAME}<br>	∧ +2.reserved not in {=, >}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR}<br>	∧ ^2.internal_type not in {File}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 12648.` |
| 74 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 3593.` |
| 75 | `  -1.roles not in {STRING}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 3819.` |
| 76 | `  -1.roles in {IDENTIFIER}<br>	∧ +3.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.946. Support: 640.` |
| 77 | `  -1.roles in {IDENTIFIER}<br>	∧ +3.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {VARIABLE} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 358.` |
| 78 | `  -1.roles in {IDENTIFIER}<br>	∧ +3.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 380.` |
| 79 | `  -1.roles in {IDENTIFIER}<br>	∧ +2.reserved = ><br>	∧ +3.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 139.` |
| 80 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved not in {>}<br>	∧ +3.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.939. Support: 4228.` |
| 81 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 1459.` |
| 82 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.975. Support: 257.` |
| 83 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.931. Support: 689.` |
| 84 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 820.` |
| 85 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 702.` |
| 86 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved = ><br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 181.` |
| 87 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 3896.` |
| 88 | `  -1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {INCOMPLETE} and not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.976. Support: 232.` |
| 89 | `  -1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.938. Support: 748.` |
| 90 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {JSXAttribute, JSXElement}<br>	∧ ^1.roles not in {CALL, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 596.` |
| 91 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {IDENTIFIER}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 934.` |
| 92 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 294.` |
| 93 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ +2.roles not in {FUNCTION}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 146.` |
| 94 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.921. Support: 422.` |
| 95 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles not in {INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.928. Support: 741.` |
| 96 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -2.roles not in {INCOMPLETE}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = '<br>Confidence: 0.931. Support: 574.` |
| 97 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.991. Support: 3951.` |
| 98 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 3475.` |
| 99 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {IDENTIFIER} and not in {LITERAL}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 876.` |
| 100 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {IDENTIFIER, LITERAL}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {DECLARATION} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 276.` |
| 101 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = ><br>	∧ +1.roles not in {IDENTIFIER, LITERAL}<br>	∧ +2.roles not in {FUNCTION}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {DECLARATION} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 148.` |
| 102 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.roles in {INCOMPLETE}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {IDENTIFIER, LITERAL}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 160.` |
| 103 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {IDENTIFIER, LITERAL}<br>	∧ ^1.internal_type not in {ConditionalExpression, JSXElement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 3581.` |
| 104 | `  -1.roles in {IDENTIFIER}<br>	∧ +3.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.937. Support: 403.` |
| 105 | `  -1.roles in {IDENTIFIER}<br>	∧ +3.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {VARIABLE} and not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 344.` |
| 106 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +3.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {ArrowFunctionExpression, JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 4173.` |
| 107 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {INCOMPLETE} and not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.985. Support: 240.` |
| 108 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.roles in {INCOMPLETE}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {STRING} and not in {COMMENT}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.923. Support: 576.` |
| 109 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.roles in {INCOMPLETE}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 265.` |
| 110 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.roles not in {INCOMPLETE}<br>	∧ +1.reserved = ,<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 214.` |
| 111 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.roles not in {INCOMPLETE}<br>	∧ +1.reserved = ><br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 207.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.063063063063064, "max_conf": 0.9992877244949341, "max_support": 13348, "min_conf": 0.920616090297699, "min_support": 139, "num_rules": 111}}
```
</details>
