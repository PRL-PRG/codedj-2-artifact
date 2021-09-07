# Model report for file:///tmp/top-repos-quality-repos-nbv4mtwe/pedwayapp.git HEAD c444e47e59396d7c68f58277ec70f494e97ad7f5

### Dump

```json
{'created_at': '2021-09-02 00:14:42',
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
 'size': '19.9 kB',
 'tags': [],
 'uuid': 'b4bf04c6-a4f0-4261-bdc5-f905c60b5e0f',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-nbv4mtwe/pedwayapp.git c444e47e59396d7c68f58277ec70f494e97ad7f5

# javascript
139 rules, avg.len. 8.6
## train
PPCR: 0.974032
### report
macro
{'f1-score': 0.6078653892871733,
 'precision': 0.6901421312597088,
 'recall': 0.5704013704721463,
 'support': 25656}
micro
{'f1-score': 0.880651699407546,
 'precision': 0.880651699407546,
 'recall': 0.880651699407546,
 'support': 25656}
weighted
{'f1-score': 0.8658982800618449,
 'precision': 0.8737444153237094,
 'recall': 0.880651699407546,
 'support': 25656}
### report_full
macro
{'f1-score': 0.5925088511994272,
 'precision': 0.6901421312597088,
 'recall': 0.556294612045518,
 'support': 26340}
micro
{'f1-score': 0.8690668512962536,
 'precision': 0.880651699407546,
 'recall': 0.8577828397873956,
 'support': 26340}
weighted
{'f1-score': 0.8458475701391867,
 'precision': 0.8694500897772728,
 'recall': 0.8577828397873956,
 'support': 26340}
## test
PPCR: 0.976533
### report
macro
{'f1-score': 0.6117760929894613,
 'precision': 0.6938879068780173,
 'recall': 0.5753131723182049,
 'support': 6450}
micro
{'f1-score': 0.8824806201550388,
 'precision': 0.8824806201550388,
 'recall': 0.8824806201550388,
 'support': 6450}
weighted
{'f1-score': 0.8688508004485961,
 'precision': 0.8778509690791613,
 'recall': 0.8824806201550388,
 'support': 6450}
### report_full
macro
{'f1-score': 0.5977438763028957,
 'precision': 0.6938879068780173,
 'recall': 0.5608483505182439,
 'support': 6605}
micro
{'f1-score': 0.8720030639601686,
 'precision': 0.8824806201550388,
 'recall': 0.861771385314156,
 'support': 6605}
weighted
{'f1-score': 0.8507194868048259,
 'precision': 0.8733740263573382,
 'recall': 0.861771385314156,
 'support': 6605}
```

## javascript
### Summary
85 rules, avg.len. 8.1

| | |
|-|-|
|Min support|140|
|Max support|4891|
|Min confidence|0.9203316569328308|
|Max confidence|0.9997212886810303|

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
                     'max_features': 'auto',
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
| 1 | `  -1.roles in {EXPRESSION}<br>	∧ -1.length ≤ 23<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = AssignmentExpression<br>	∧ ^1.roles not in {MODULE, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 159.` |
| 2 | `  -1.roles in {EXPRESSION, STRING}<br>	∧ -1.length ≤ 23<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {AssignmentExpression, ObjectExpression}<br>	∧ ^1.roles not in {MODULE, VARIABLE}<br>⇒ y = '<br>Confidence: 0.999. Support: 483.` |
| 3 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ -1.length ≤ 23<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {AssignmentExpression, ObjectExpression}<br>	∧ ^1.roles not in {MODULE, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 4891.` |
| 4 | `  -1.reserved = .<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1794.` |
| 5 | `  -1.reserved not in {., ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type = VariableDeclarator<br>⇒ y = ␣<br>Confidence: 0.984. Support: 284.` |
| 6 | `  •••start_line ≥ 41<br>	∧ -1.diff_offset ≥ 3<br>	∧ -1.reserved not in {., ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved = {<br>	∧ +2.length ≤ 3<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 160.` |
| 7 | `  -1.diff_offset ≤ 2<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_col ≥ 3<br>	∧ -4.internal_type not in {Identifier}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +3.reserved not in {{}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 849.` |
| 8 | `  -1.diff_offset ≤ 2<br>	∧ -1.reserved not in {(, ., ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_col ≥ 3<br>	∧ -4.internal_type not in {Identifier}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +3.reserved not in {{}<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 183.` |
| 9 | `  -1.diff_offset ≤ 2<br>	∧ -1.reserved not in {., ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_col ≤ 2<br>	∧ -3.diff_offset ≤ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved = (<br>	∧ +3.reserved not in {{}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.989. Support: 140.` |
| 10 | `  -1.diff_offset ≤ 2<br>	∧ -1.reserved not in {., ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_col ≤ 2<br>	∧ -3.diff_offset ≤ 3<br>	∧ -3.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {(}<br>	∧ +2.length ≥ 2<br>	∧ +3.reserved not in {{}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 192.` |
| 11 | `  -1.roles in {STRING}<br>⇒ y = '<br>Confidence: 0.999. Support: 861.` |
| 12 | `  -1.roles not in {STRING}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.957. Support: 595.` |
| 13 | `  -1.reserved not in {,}<br>	∧ -1.roles not in {STRING}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.937. Support: 4385.` |
| 14 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = '<br>Confidence: 0.998. Support: 283.` |
| 15 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {CALL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 583.` |
| 16 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 758.` |
| 17 | `  -1.reserved = ,<br>	∧ +2.reserved = :<br>⇒ y = ⏎<br>Confidence: 0.932. Support: 302.` |
| 18 | `  -1.reserved not in {,}<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -4.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {LITERAL}<br>	∧ ^2.internal_type not in {File}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 147.` |
| 19 | `  -1.reserved not in {,}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -4.roles not in {IDENTIFIER}<br>	∧ -5.internal_type = Identifier<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^2.internal_type not in {File}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.923. Support: 176.` |
| 20 | `  -1.reserved not in {,}<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.internal_type = Identifier<br>	∧ -4.roles not in {IDENTIFIER}<br>	∧ -5.internal_type not in {Identifier}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^2.internal_type not in {File}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 345.` |
| 21 | `  -1.reserved = .<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved not in {}}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 1797.` |
| 22 | `  -1.reserved not in {,, .}<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles in {MAP}<br>	∧ +1.reserved not in {}}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 525.` |
| 23 | `  -1.reserved not in {,, .}<br>	∧ -2.label not in {<space>}<br>	∧ -2.reserved not in {)}<br>	∧ +1.internal_type = Identifier<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved = =<br>⇒ y = ␣<br>Confidence: 0.985. Support: 239.` |
| 24 | `  -1.reserved not in {,, .}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.label not in {<space>}<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.label not in {<space>}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {ArrowFunctionExpression}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 785.` |
| 25 | `  -1.reserved not in {,, .}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.label not in {<space>}<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.label not in {<space>}<br>	∧ -3.roles not in {LITERAL}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved not in {,}<br>	∧ ^1.internal_type not in {ArrowFunctionExpression}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>	∧ ^2.internal_type not in {BlockStatement}<br>	∧ ^2.roles in {INITIALIZATION}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 504.` |
| 26 | `  -1.reserved not in {,, .}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.label not in {<space>}<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.label not in {<space>}<br>	∧ -3.roles not in {LITERAL}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {,, =}<br>	∧ ^1.internal_type not in {ArrowFunctionExpression}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>	∧ ^2.internal_type not in {BlockStatement}<br>	∧ ^2.roles not in {INITIALIZATION}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 1346.` |
| 27 | `  +1.roles in {STRING}<br>	∧ ^2.internal_type = ExpressionStatement<br>⇒ y = '<br>Confidence: 0.931. Support: 197.` |
| 28 | `  -1.reserved = (<br>	∧ -1.roles not in {CALL}<br>	∧ -2.diff_offset ≥ 3<br>	∧ +1.roles not in {STRING}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 506.` |
| 29 | `  -1.reserved not in {)}<br>	∧ -2.diff_offset ≤ 2<br>	∧ +1.roles not in {STRING}<br>	∧ ^2.internal_type = VariableDeclaration<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 211.` |
| 30 | `  -1.reserved = (<br>	∧ -2.diff_offset ≤ 2<br>	∧ -3.diff_col ≤ 11<br>	∧ +1.roles not in {STRING}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 167.` |
| 31 | `  -2.roles in {ARGUMENT} and not in {MAP}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.roles not in {STRING}<br>	∧ +2.length ≤ 36<br>	∧ +3.reserved = {<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 163.` |
| 32 | `  -1.roles in {IDENTIFIER}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.roles not in {STRING}<br>	∧ +2.length ≤ 36<br>	∧ +3.reserved not in {{}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 3662.` |
| 33 | `  -1.diff_offset ≥ 5<br>	∧ -1.roles in {STRING} and not in {ARGUMENT, IDENTIFIER}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -4.diff_offset ≤ 34<br>	∧ +1.roles not in {KEY, STRING}<br>	∧ +2.length ≤ 36<br>	∧ +3.reserved not in {{}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = '<br>Confidence: 0.998. Support: 271.` |
| 34 | `  -1.diff_offset ≥ 5<br>	∧ -1.roles not in {ARGUMENT, IDENTIFIER, STRING}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -4.diff_offset ≤ 34<br>	∧ +1.roles not in {KEY, STRING}<br>	∧ +1.length ≥ 3<br>	∧ +2.length ≤ 36<br>	∧ +3.reserved not in {{}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 217.` |
| 35 | `  -1.reserved = {<br>	∧ -2.label in {<space>}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.diff_offset ≤ 3<br>	∧ +1.roles not in {STRING}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.936. Support: 369.` |
| 36 | `  -1.internal_type = StringLiteral<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = '<br>Confidence: 0.997. Support: 143.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.939. Support: 4373.` |
| 38 | `  -1.reserved = {<br>	∧ +1.reserved not in {(}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.948. Support: 300.` |
| 39 | `  -1.reserved not in {,, {}<br>	∧ -4.diff_offset ≥ 15<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles in {STATEMENT, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 265.` |
| 40 | `  -1.reserved not in {,, ;, {}<br>	∧ -4.diff_offset ≤ 14<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.internal_type = VariableDeclaration<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 225.` |
| 41 | `  •••start_col ≤ 29<br>	∧ -1.reserved = (<br>	∧ -4.diff_offset ≤ 14<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(}<br>	∧ +2.internal_type not in {Identifier}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 188.` |
| 42 | `  -1.reserved not in {,, {}<br>	∧ -1.roles not in {CALL}<br>	∧ -5.roles not in {IMPORT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(}<br>	∧ +1.roles in {EXPRESSION} and not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {CALL}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 448.` |
| 43 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,, {}<br>	∧ -1.roles not in {CALL}<br>	∧ -5.roles not in {IMPORT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(}<br>	∧ +1.roles not in {EXPRESSION, LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.internal_type not in {ClassBody}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = '<br>Confidence: 0.998. Support: 298.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, {}<br>	∧ -1.roles not in {CALL}<br>	∧ -1.length ≥ 2<br>	∧ -4.roles in {MAP}<br>	∧ -5.roles not in {IMPORT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(}<br>	∧ +1.roles not in {EXPRESSION, LITERAL}<br>	∧ +2.length ≤ 30<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.internal_type not in {ClassBody}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 318.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, {}<br>	∧ -1.roles not in {CALL}<br>	∧ -1.length ≥ 2<br>	∧ -4.roles not in {MAP}<br>	∧ -5.diff_line ≥ 1<br>	∧ -5.roles not in {IMPORT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(}<br>	∧ +1.roles not in {EXPRESSION, LITERAL}<br>	∧ +2.length ≤ 30<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.internal_type not in {ClassBody}<br>	∧ ^2.roles in {MAP} and not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 229.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, {}<br>	∧ -1.roles not in {CALL}<br>	∧ -1.length ≤ 1<br>	∧ -5.roles in {FUNCTION} and not in {IMPORT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {MODULE}<br>	∧ ^2.internal_type not in {ClassBody}<br>	∧ ^2.roles in {EXPRESSION} and not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 151.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, {}<br>	∧ -1.roles not in {CALL}<br>	∧ -1.length ≤ 1<br>	∧ -5.roles not in {IMPORT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(}<br>	∧ +1.roles not in {EXPRESSION, LITERAL}<br>	∧ +2.length ≤ 30<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {MODULE}<br>	∧ ^2.internal_type not in {ClassBody}<br>	∧ ^2.roles in {BLOCK} and not in {EXPRESSION, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 204.` |
| 48 | `  -1.roles in {CALL} and not in {STRING}<br>	∧ +1.roles not in {STRING}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 907.` |
| 49 | `  -1.roles not in {CALL, STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {DECLARATION}<br>	∧ ^2.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 538.` |
| 50 | `  -1.roles in {ARGUMENT} and not in {CALL, STRING}<br>	∧ -2.length ≤ 2<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {DECLARATION}<br>	∧ ^2.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 276.` |
| 51 | `  -1.roles not in {CALL, STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {IDENTIFIER} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 4184.` |
| 52 | `  -1.roles not in {CALL, STRING}<br>	∧ +1.roles in {EXPRESSION, VALUE} and not in {KEY, STRING}<br>	∧ +2.reserved not in {(}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.924. Support: 231.` |
| 53 | `  -1.roles not in {CALL, STRING}<br>	∧ -1.length ≤ 1<br>	∧ +1.roles not in {EXPRESSION, STRING}<br>	∧ +2.roles in {IDENTIFIER}<br>	∧ +5.reserved not in {(}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 216.` |
| 54 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.999. Support: 885.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = }<br>⇒ y = ∅<br>Confidence: 0.997. Support: 185.` |
| 56 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {IDENTIFIER} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 4283.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = const<br>	∧ -2.reserved not in {)}<br>	∧ -3.diff_col ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {VALUE}<br>	∧ +2.reserved not in {:}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 195.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, const}<br>	∧ -2.reserved not in {)}<br>	∧ -3.diff_col ≥ 4<br>	∧ -5.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved not in {:}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.984. Support: 395.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, const}<br>	∧ -2.reserved not in {)}<br>	∧ -3.diff_col ≥ 4<br>	∧ -5.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {VALUE}<br>	∧ +2.reserved not in {:}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type = JSXOpeningElement<br>⇒ y = ∅<br>Confidence: 0.997. Support: 159.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, const}<br>	∧ -2.reserved not in {)}<br>	∧ -3.diff_col ≥ 4<br>	∧ -5.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION} and not in {VALUE}<br>	∧ +2.reserved not in {:}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles in {CALLEE}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 263.` |
| 61 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.diff_offset ≤ 2<br>	∧ -2.reserved not in {)}<br>	∧ -3.diff_col ≤ 3<br>	∧ -5.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ +1.roles not in {VALUE}<br>	∧ +2.reserved not in {:}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 201.` |
| 62 | `  -1.reserved = this<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 383.` |
| 63 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {this}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 150.` |
| 64 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {this}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 4002.` |
| 65 | `  -1.internal_type = Identifier<br>	∧ -1.reserved not in {;}<br>	∧ +2.roles in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 807.` |
| 66 | `  •••start_col ≥ 33<br>	∧ -1.internal_type = Identifier<br>	∧ -1.reserved not in {;}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {ASSIGNMENT, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 888.` |
| 67 | `  •••start_col ≤ 32<br>	∧ -1.internal_type = Identifier<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = =<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {ASSIGNMENT, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 200.` |
| 68 | `  •••start_col ≤ 32<br>	∧ -1.internal_type = Identifier<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved not in {=}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {ASSIGNMENT, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.920. Support: 1387.` |
| 69 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.999. Support: 675.` |
| 70 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ ^1.roles in {VARIABLE} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 305.` |
| 71 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved = =<br>	∧ -5.roles not in {FUNCTION}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {:}<br>	∧ ^1.roles not in {IDENTIFIER, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 140.` |
| 72 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = const<br>	∧ -1.roles not in {STRING}<br>	∧ -5.roles not in {FUNCTION}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {:}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 178.` |
| 73 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ -2.roles not in {MAP}<br>	∧ -5.roles not in {FUNCTION}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {:}<br>	∧ ^1.roles not in {IDENTIFIER, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 1502.` |
| 74 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {(, ,, ;, const, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.roles not in {MAP}<br>	∧ -5.roles not in {FUNCTION}<br>	∧ +1.roles not in {IDENTIFIER, LITERAL}<br>	∧ +2.reserved not in {:, =}<br>	∧ ^1.internal_type not in {AssignmentExpression}<br>	∧ ^1.roles in {INCOMPLETE} and not in {BODY, IDENTIFIER, VARIABLE}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>	∧ ^2.roles not in {FUNCTION}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 816.` |
| 75 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.943. Support: 4369.` |
| 76 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles in {VARIABLE} and not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.roles not in {LITERAL, MODULE}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 254.` |
| 77 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR, VARIABLE}<br>	∧ ^2.roles not in {LITERAL, MODULE}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 210.` |
| 78 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles in {ARGUMENT}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles in {FUNCTION}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 173.` |
| 79 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -2.diff_col ≤ 2<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {BLOCK} and not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 233.` |
| 80 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_col ≤ 2<br>	∧ -2.reserved not in {'}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -3.diff_col ≥ 3<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {BLOCK, IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.950. Support: 212.` |
| 81 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 171.` |
| 82 | `  -1.diff_col ≤ 4<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 2997.` |
| 83 | `  •••start_line ≥ 50<br>	∧ -1.diff_col ≤ 4<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.roles not in {VALUE}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {BlockStatement, MemberExpression}<br>	∧ ^1.roles not in {BLOCK, DECLARATION}<br>	∧ ^2.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 587.` |
| 84 | `  -1.diff_col ≤ 4<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.roles not in {VALUE}<br>	∧ -4.internal_type not in {Identifier}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BlockStatement, MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, STATEMENT}<br>	∧ ^2.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 303.` |
| 85 | `  -1.diff_col ≤ 4<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.roles not in {VALUE}<br>	∧ -4.internal_type not in {Identifier}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BlockStatement, MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, STATEMENT}<br>	∧ ^2.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 369.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.094117647058823, "max_conf": 0.9997212886810303, "max_support": 4891, "min_conf": 0.9203316569328308, "min_support": 140, "num_rules": 85}}
```
</details>
