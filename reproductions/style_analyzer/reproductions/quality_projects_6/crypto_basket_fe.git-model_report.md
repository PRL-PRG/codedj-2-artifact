# Model report for file:///tmp/top-repos-quality-repos-xd9j98gp/crypto_basket_fe.git HEAD ef0ee505b80546a0087899dca26ce3ff83438de7

### Dump

```json
{'created_at': '2021-09-02 00:55:23',
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
 'size': '23.0 kB',
 'tags': [],
 'uuid': '84636622-1ba9-4eea-ae8d-730769eafe4c',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-xd9j98gp/crypto_basket_fe.git ef0ee505b80546a0087899dca26ce3ff83438de7

# javascript
305 rules, avg.len. 9.2
## train
PPCR: 0.976318
### report
macro
{'f1-score': 0.8680213855367652,
 'precision': 0.90682543467972,
 'recall': 0.8372281038061361,
 'support': 73549}
micro
{'f1-score': 0.9431127547621314,
 'precision': 0.9431127547621314,
 'recall': 0.9431127547621314,
 'support': 73549}
weighted
{'f1-score': 0.9419783493664302,
 'precision': 0.9422677113328626,
 'recall': 0.9431127547621314,
 'support': 73549}
### report_full
macro
{'f1-score': 0.8280769615086856,
 'precision': 0.90682543467972,
 'recall': 0.7788049456961689,
 'support': 75333}
micro
{'f1-score': 0.9318117703953467,
 'precision': 0.9431127547621314,
 'recall': 0.9207784105239404,
 'support': 75333}
weighted
{'f1-score': 0.9287162846473556,
 'precision': 0.9412408868573429,
 'recall': 0.9207784105239404,
 'support': 75333}
## test
PPCR: 0.975061
### report
macro
{'f1-score': 0.8533648817219696,
 'precision': 0.9015680804616366,
 'recall': 0.8183633110928668,
 'support': 17946}
micro
{'f1-score': 0.9397637356513986,
 'precision': 0.9397637356513986,
 'recall': 0.9397637356513986,
 'support': 17946}
weighted
{'f1-score': 0.9384109424241616,
 'precision': 0.9389628130152419,
 'recall': 0.9397637356513986,
 'support': 17946}
### report_full
macro
{'f1-score': 0.8110048149050157,
 'precision': 0.9015680804616366,
 'recall': 0.7608344709162451,
 'support': 18405}
micro
{'f1-score': 0.9278974443619157,
 'precision': 0.9397637356513986,
 'recall': 0.9163270850312415,
 'support': 18405}
weighted
{'f1-score': 0.9239360715802641,
 'precision': 0.9373602861103927,
 'recall': 0.9163270850312415,
 'support': 18405}
```

## javascript
### Summary
189 rules, avg.len. 9.0

| | |
|-|-|
|Min support|135|
|Max support|9983|
|Min confidence|0.9214411377906799|
|Max confidence|0.9996471405029297|

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
| 1 | `  -1.internal_type = StringLiteral<br>	∧ +2.roles in {UNANNOTATED}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = "<br>Confidence: 0.993. Support: 614.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.982. Support: 9715.` |
| 3 | `  ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 5952.` |
| 4 | `  -1.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ +5.roles not in {MAP}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.970. Support: 1057.` |
| 5 | `  -1.label not in {<space>}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.996. Support: 948.` |
| 6 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.991. Support: 266.` |
| 7 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 1249.` |
| 8 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.roles in {INCOMPLETE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 491.` |
| 9 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.roles not in {INCOMPLETE}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.957. Support: 853.` |
| 10 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.label not in {<space>}<br>	∧ +1.roles not in {ARGUMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {CALL, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.954. Support: 1021.` |
| 11 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = <<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.939. Support: 223.` |
| 12 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -4.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {CallExpression, File, JSXElement}<br>	∧ ^1.roles not in {BLOCK, LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 271.` |
| 13 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, <, {, }}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.diff_offset ≥ 6<br>	∧ -4.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {CallExpression, File, JSXElement}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED, VALUE}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 5854.` |
| 14 | `  •••start_col ≤ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.931. Support: 573.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 374.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {VARIABLE} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 1197.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_offset ≤ 3<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {VARIABLE} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.928. Support: 201.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -3.diff_offset ≤ 3<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {VARIABLE} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 192.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = ><br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 207.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {ConditionalExpression, JSXElement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 7490.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED, VARIABLE}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1390.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = ;<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 172.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 746.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = =<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 1085.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {=}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 716.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, =}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -1.length ≥ 8<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 280.` |
| 27 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, =}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -1.length ≤ 7<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 1313.` |
| 28 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, =}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -1.length ≤ 7<br>	∧ +1.reserved not in {), ,, ;, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {DECLARATION} and not in {INCOMPLETE, QUALIFIED, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 397.` |
| 29 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, =}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -1.length ≤ 7<br>	∧ +1.reserved not in {), ,, ;, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, QUALIFIED, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 505.` |
| 30 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -1.length ≤ 1<br>	∧ +1.reserved not in {), ,, ;, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, QUALIFIED, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 264.` |
| 31 | `  -1.internal_type = StringLiteral<br>	∧ +1.reserved = ><br>	∧ ^1.internal_type = JSXElement<br>⇒ y = "<br>Confidence: 0.999. Support: 713.` |
| 32 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.985. Support: 240.` |
| 33 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 1263.` |
| 34 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.internal_type = JSXIdentifier<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 433.` |
| 35 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.internal_type not in {JSXIdentifier}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.965. Support: 907.` |
| 36 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = )<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 265.` |
| 37 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.label not in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {)}<br>	∧ ^1.internal_type not in {CallExpression, JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.939. Support: 1156.` |
| 38 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 367.` |
| 39 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = <<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, JSXElement}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 202.` |
| 40 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.diff_offset ≥ 6<br>	∧ -4.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {CallExpression, File, JSXElement}<br>	∧ ^1.roles not in {BLOCK, LITERAL, QUALIFIED, VALUE}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 271.` |
| 41 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, <, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.diff_offset ≥ 6<br>	∧ -4.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {CallExpression, File, JSXElement}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED, VALUE}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 5882.` |
| 42 | `  •••start_col ≤ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 543.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {INCOMPLETE}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 471.` |
| 44 | `  •••start_col ≥ 54<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.997. Support: 150.` |
| 45 | `  •••start_col ≤ 53<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.958. Support: 1146.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED, VARIABLE}<br>	∧ ^2.internal_type = JSXOpeningElement<br>⇒ y = ∅<br>Confidence: 0.990. Support: 634.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED, VARIABLE}<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 371.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED, VARIABLE}<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 1321.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 250.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {CONDITION} and not in {QUALIFIED, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 166.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {INCOMPLETE}<br>	∧ +2.length ≥ 1<br>	∧ +5.roles not in {STRING}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {CONDITION, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 4429.` |
| 52 | `  ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 5977.` |
| 53 | `  -1.roles not in {COMMENT, STRING}<br>	∧ -1.length ≥ 3<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, MAP}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 4114.` |
| 54 | `  -1.length ≤ 2<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.999. Support: 918.` |
| 55 | `  -1.reserved = :<br>	∧ -1.length ≤ 2<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 312.` |
| 56 | `  -1.reserved not in {:}<br>	∧ -1.length ≤ 2<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.948. Support: 144.` |
| 57 | `  -1.reserved not in {:}<br>	∧ -1.length ≤ 2<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ +5.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.931. Support: 1291.` |
| 58 | `  -1.reserved = (<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 1304.` |
| 59 | `  -1.reserved = {<br>	∧ -1.length ≤ 2<br>	∧ -3.internal_type = JSXIdentifier<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 430.` |
| 60 | `  -1.reserved = {<br>	∧ -1.length ≤ 2<br>	∧ -3.internal_type not in {JSXIdentifier}<br>	∧ +1.internal_type not in {Identifier, StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.958. Support: 884.` |
| 61 | `  -1.reserved not in {(, ,, ;, {}<br>	∧ -1.length ≤ 2<br>	∧ -2.label in {<newline>} and not in {<-space>}<br>	∧ -3.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {BLOCK}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 478.` |
| 62 | `  -1.reserved not in {(, ,, ;, {}<br>	∧ -1.length ≤ 2<br>	∧ -2.label not in {<-space>, <newline>}<br>	∧ -3.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {INCOMPLETE} and not in {BLOCK}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 332.` |
| 63 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 393.` |
| 64 | `  •••start_col ≥ 52<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.997. Support: 148.` |
| 65 | `  •••start_col ≤ 51<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.966. Support: 1145.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {VARIABLE} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 1175.` |
| 67 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -3.diff_offset ≤ 3<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {VARIABLE} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 193.` |
| 68 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = ><br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 199.` |
| 69 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {ConditionalExpression, JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 7542.` |
| 70 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, VARIABLE}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1417.` |
| 71 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = ;<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 189.` |
| 72 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 702.` |
| 73 | `  -1.diff_offset ≥ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, =}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 257.` |
| 74 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.997. Support: 5903.` |
| 75 | `  -1.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ +5.roles not in {VALUE}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = '<br>Confidence: 0.957. Support: 1120.` |
| 76 | `  -1.label not in {<space>}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = "<br>Confidence: 0.996. Support: 940.` |
| 77 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = '<br>Confidence: 0.970. Support: 251.` |
| 78 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 1275.` |
| 79 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.roles not in {INCOMPLETE}<br>	∧ +1.internal_type = Identifier<br>	∧ +1.length ≥ 2<br>	∧ +4.roles in {PATHNAME}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 151.` |
| 80 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.roles not in {INCOMPLETE}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.939. Support: 887.` |
| 81 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = )<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 271.` |
| 82 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 309.` |
| 83 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = <<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 192.` |
| 84 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.diff_offset ≥ 6<br>	∧ -4.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {BLOCK, FILE, LITERAL, VALUE}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 216.` |
| 85 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, <, {, }}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.diff_offset ≥ 6<br>	∧ -4.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {FILE, LITERAL, VALUE}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 5876.` |
| 86 | `  •••start_col ≤ 6<br>	∧ -1.diff_col ≤ 4<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 672.` |
| 87 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {INCOMPLETE}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 481.` |
| 88 | `  •••start_col ≥ 54<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_col ≤ 2<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.956. Support: 171.` |
| 89 | `  •••start_col ≤ 53<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.957. Support: 1197.` |
| 90 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 1241.` |
| 91 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_offset ≤ 3<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.943. Support: 237.` |
| 92 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -3.diff_offset ≤ 3<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 224.` |
| 93 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = ><br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 202.` |
| 94 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {ConditionalExpression, JSXElement, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 7576.` |
| 95 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {VARIABLE}<br>	∧ ^2.internal_type = JSXOpeningElement<br>⇒ y = ∅<br>Confidence: 0.991. Support: 593.` |
| 96 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {VARIABLE}<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 395.` |
| 97 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {VARIABLE}<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 1350.` |
| 98 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 250.` |
| 99 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles in {CONDITION} and not in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 174.` |
| 100 | `  -1.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ +5.roles not in {VALUE}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.960. Support: 1096.` |
| 101 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.975. Support: 262.` |
| 102 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 1307.` |
| 103 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.997. Support: 893.` |
| 104 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = <<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, JSXElement}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 196.` |
| 105 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -4.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {CallExpression, File, JSXElement}<br>	∧ ^1.roles not in {BLOCK, LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 271.` |
| 106 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, <, {, }}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.diff_offset ≥ 6<br>	∧ -4.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {CallExpression, File, JSXElement}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED, VALUE}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 5924.` |
| 107 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {VARIABLE} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 1212.` |
| 108 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -3.diff_offset ≤ 3<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {VARIABLE} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 190.` |
| 109 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {INCOMPLETE}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 531.` |
| 110 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ -4.length ≤ 6<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 383.` |
| 111 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = ;<br>	∧ +2.roles not in {INCOMPLETE}<br>	∧ +2.length ≥ 1<br>	∧ +5.roles not in {STRING}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {CONDITION, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 154.` |
| 112 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {;}<br>	∧ +2.roles not in {INCOMPLETE}<br>	∧ +2.length ≥ 1<br>	∧ +5.roles not in {STRING}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {CONDITION, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.932. Support: 4087.` |
| 113 | `  -1.diff_offset ≥ 3<br>	∧ -1.roles in {STRING}<br>	∧ +1.length ≥ 7<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.997. Support: 192.` |
| 114 | `  -1.diff_offset ≥ 3<br>	∧ -1.roles not in {COMMENT, STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {MAP, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 4153.` |
| 115 | `  -1.diff_offset ≤ 2<br>	∧ -2.roles in {INCOMPLETE}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.999. Support: 925.` |
| 116 | `  -1.diff_offset ≤ 2<br>	∧ -1.reserved = :<br>	∧ -2.roles not in {INCOMPLETE}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 303.` |
| 117 | `  -1.diff_offset ≤ 2<br>	∧ -1.reserved not in {:}<br>	∧ -2.label in {<space>}<br>	∧ -2.roles not in {INCOMPLETE}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 158.` |
| 118 | `  -1.diff_offset ≤ 2<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 1209.` |
| 119 | `  -1.diff_offset ≤ 2<br>	∧ -1.reserved = {<br>	∧ -3.roles in {INCOMPLETE}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 473.` |
| 120 | `  -1.diff_offset ≤ 2<br>	∧ -1.reserved = {<br>	∧ -3.roles not in {INCOMPLETE}<br>	∧ +1.internal_type = Identifier<br>	∧ +1.roles in {IMPORT}<br>	∧ +1.length ≥ 2<br>	∧ +5.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 173.` |
| 121 | `  -1.diff_offset ≤ 2<br>	∧ -1.reserved = {<br>	∧ -3.roles not in {INCOMPLETE}<br>	∧ +1.internal_type not in {Identifier, StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.948. Support: 892.` |
| 122 | `  -1.diff_offset ≤ 2<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = )<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 232.` |
| 123 | `  •••start_col ≤ 11<br>	∧ -1.diff_col ≥ 2<br>	∧ -1.diff_offset ≤ 2<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -3.length ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 186.` |
| 124 | `  -1.diff_col ≤ 1<br>	∧ -1.diff_offset ≤ 2<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -3.length ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 1809.` |
| 125 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = JSXAttribute<br>	∧ ^1.roles not in {QUALIFIED, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 626.` |
| 126 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXAttribute, JSXElement}<br>	∧ ^1.roles not in {QUALIFIED, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 422.` |
| 127 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXAttribute, JSXElement}<br>	∧ ^1.roles not in {QUALIFIED, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 1352.` |
| 128 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {const}<br>	∧ +5.roles not in {STRING}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {CONDITION, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.921. Support: 4538.` |
| 129 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length = 0<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {CONDITION, QUALIFIED, VARIABLE}<br>⇒ y = ⏎<br>Confidence: 0.922. Support: 173.` |
| 130 | `  -1.internal_type = StringLiteral<br>	∧ +2.internal_type = JSXText<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = "<br>Confidence: 0.989. Support: 569.` |
| 131 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.974. Support: 9983.` |
| 132 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = <<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.929. Support: 217.` |
| 133 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, <, {, }}<br>	∧ -1.roles not in {COMMENT, STRING}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.diff_offset ≥ 6<br>	∧ -4.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {ArrowFunctionExpression, CallExpression, JSXElement}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 6267.` |
| 134 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 444.` |
| 135 | `  •••start_col ≥ 36<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.989. Support: 330.` |
| 136 | `  •••start_col ≤ 35<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.973. Support: 902.` |
| 137 | `  •••start_col ≥ 7<br>	∧ -1.diff_offset ≤ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, =}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -3.internal_type not in {JSXIdentifier}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 1352.` |
| 138 | `  •••start_col ≥ 7<br>	∧ -1.diff_offset ≤ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, =}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -3.internal_type not in {JSXIdentifier}<br>	∧ -4.diff_offset ≤ 19<br>	∧ +1.reserved not in {), ,, ;, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {DECLARATION} and not in {QUALIFIED, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 391.` |
| 139 | `  •••start_col ≥ 7<br>	∧ -1.diff_offset ≤ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, =}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -3.internal_type not in {JSXIdentifier}<br>	∧ -4.diff_offset ≤ 19<br>	∧ +1.internal_type = NumericLiteral<br>	∧ +1.reserved not in {), ,, ;, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 221.` |
| 140 | `  -1.diff_offset ≥ 3<br>	∧ -1.roles in {STRING}<br>	∧ +1.length ≥ 7<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = "<br>Confidence: 0.997. Support: 187.` |
| 141 | `  -1.diff_offset ≥ 3<br>	∧ -1.roles not in {COMMENT, STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 4190.` |
| 142 | `  -1.diff_offset ≤ 2<br>	∧ -2.roles in {INCOMPLETE}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = "<br>Confidence: 0.999. Support: 938.` |
| 143 | `  -1.diff_offset ≤ 2<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -2.roles not in {INCOMPLETE}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = '<br>Confidence: 0.971. Support: 228.` |
| 144 | `  -1.diff_offset ≤ 2<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 1371.` |
| 145 | `  -1.diff_offset ≤ 2<br>	∧ -1.reserved = {<br>	∧ -3.internal_type = JSXIdentifier<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 461.` |
| 146 | `  -1.diff_offset ≤ 2<br>	∧ -1.reserved = {<br>	∧ -3.internal_type not in {JSXIdentifier}<br>	∧ +1.internal_type = Identifier<br>	∧ +1.length ≥ 2<br>	∧ +4.roles in {PATHNAME}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 144.` |
| 147 | `  -1.diff_offset ≤ 2<br>	∧ -1.reserved = {<br>	∧ -3.internal_type not in {JSXIdentifier}<br>	∧ +1.internal_type not in {Identifier, StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.937. Support: 906.` |
| 148 | `  -1.diff_offset ≤ 2<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = )<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 233.` |
| 149 | `  •••start_col ≥ 14<br>	∧ -1.diff_offset ≤ 2<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 10<br>	∧ +4.reserved not in {>}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles in {INCOMPLETE}<br>	∧ ^2.internal_type = JSXElement<br>⇒ y = ⏎<br>Confidence: 0.924. Support: 151.` |
| 150 | `  •••start_col ≥ 14<br>	∧ -1.diff_offset ≤ 2<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {INCOMPLETE}<br>	∧ ^2.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.931. Support: 255.` |
| 151 | `  •••start_col ≥ 14<br>	∧ -1.diff_offset ≤ 2<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.967. Support: 1588.` |
| 152 | `  •••start_col ≥ 14<br>	∧ -1.diff_offset ≤ 2<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -3.roles in {IDENTIFIER}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 267.` |
| 153 | `  •••start_col ≤ 13<br>	∧ -1.diff_offset ≤ 2<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -2.label in {<space>} and not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 162.` |
| 154 | `  •••start_col ≤ 13<br>	∧ -1.diff_offset ≤ 2<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -2.label not in {<-space>, <space>}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 877.` |
| 155 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {INCOMPLETE}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 463.` |
| 156 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1409.` |
| 157 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = ;<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 158.` |
| 158 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 738.` |
| 159 | `  -1.diff_offset ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, =}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 271.` |
| 160 | `  •••start_col ≥ 7<br>	∧ -1.diff_offset ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, =, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ +2.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.952. Support: 1926.` |
| 161 | `  •••start_col ≥ 7<br>	∧ -1.diff_offset ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, =, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ +2.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles in {DECLARATION} and not in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 356.` |
| 162 | `  •••start_col ≥ 7<br>	∧ -1.diff_offset ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, =, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type = NumericLiteral<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ +2.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 227.` |
| 163 | `  -1.roles in {STRING}<br>	∧ -1.length ≥ 3<br>	∧ +1.length ≥ 6<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.992. Support: 187.` |
| 164 | `  -1.roles not in {COMMENT, STRING}<br>	∧ -1.length ≥ 3<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {MAP, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 4120.` |
| 165 | `  -1.length ≤ 2<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.999. Support: 944.` |
| 166 | `  -1.reserved = :<br>	∧ -1.length ≤ 2<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 278.` |
| 167 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {:}<br>	∧ -1.length ≤ 2<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ +5.roles not in {MAP}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.958. Support: 1003.` |
| 168 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.length ≤ 2<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.974. Support: 249.` |
| 169 | `  -1.reserved = (<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 1252.` |
| 170 | `  -1.reserved = {<br>	∧ -1.length ≤ 2<br>	∧ -3.roles in {INCOMPLETE}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 476.` |
| 171 | `  -1.reserved = {<br>	∧ -1.length ≤ 2<br>	∧ -3.roles not in {INCOMPLETE}<br>	∧ +1.internal_type not in {Identifier, StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.951. Support: 921.` |
| 172 | `  -1.reserved = ,<br>	∧ -1.length ≤ 2<br>	∧ -3.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.927. Support: 942.` |
| 173 | `  •••start_col ≥ 8<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {INCOMPLETE}<br>	∧ +1.length ≥ 2<br>	∧ +4.reserved = ><br>	∧ ^1.internal_type = JSXOpeningElement<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 179.` |
| 174 | `  •••start_col ≥ 8<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {INCOMPLETE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, JSXOpeningElement}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 424.` |
| 175 | `  •••start_col ≥ 8<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 163.` |
| 176 | `  •••start_col ≤ 7<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.932. Support: 699.` |
| 177 | `  -1.internal_type = Identifier<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 366.` |
| 178 | `  -1.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ +5.roles not in {VALUE}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.965. Support: 1073.` |
| 179 | `  -1.label not in {<space>}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.995. Support: 987.` |
| 180 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.985. Support: 230.` |
| 181 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 1293.` |
| 182 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.internal_type not in {JSXIdentifier}<br>	∧ +1.internal_type = Identifier<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved = from<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.967. Support: 135.` |
| 183 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.internal_type not in {JSXIdentifier}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.930. Support: 881.` |
| 184 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.length ≤ 1<br>	∧ -4.diff_offset ≥ 8<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.995. Support: 320.` |
| 185 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.length ≤ 1<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 570.` |
| 186 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {COMMENT, STRING}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.diff_offset ≥ 6<br>	∧ -4.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {CallExpression, JSXElement}<br>	∧ ^1.roles not in {BLOCK, FILE, IDENTIFIER, LITERAL, VALUE}<br>⇒ y = ␣<br>Confidence: 0.937. Support: 262.` |
| 187 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, <, {, }}<br>	∧ -1.roles not in {COMMENT, STRING}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.diff_offset ≥ 6<br>	∧ -4.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {CallExpression, JSXElement}<br>	∧ ^1.roles not in {FILE, IDENTIFIER, LITERAL, VALUE}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 6026.` |
| 188 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {INCOMPLETE}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 469.` |
| 189 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ -5.diff_offset ≤ 21<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.946. Support: 417.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.973544973544973, "max_conf": 0.9996471405029297, "max_support": 9983, "min_conf": 0.9214411377906799, "min_support": 135, "num_rules": 189}}
```
</details>
