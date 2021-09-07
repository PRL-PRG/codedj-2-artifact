# Model report for file:///tmp/top-repos-quality-repos-panclr91/node.git HEAD e5670f49682ced87451e5fc1e56d7d3fc396c6f2

### Dump

```json
{'created_at': '2021-09-01 19:00:53',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.19.0-12-amd64-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '33.9 kB',
 'tags': [],
 'uuid': 'd96233bd-2b85-4f6f-ad39-3b004f9fff51',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-panclr91/node.git e5670f49682ced87451e5fc1e56d7d3fc396c6f2

# javascript
144 rules, avg.len. 11.6
## train
PPCR: 0.948602
### report
macro
{'f1-score': 0.35520250489919525,
 'precision': 0.3663874186611959,
 'recall': 0.3468076809681037,
 'support': 732383}
micro
{'f1-score': 0.9748697061510166,
 'precision': 0.9748697061510166,
 'recall': 0.9748697061510166,
 'support': 732383}
weighted
{'f1-score': 0.9729861940591978,
 'precision': 0.9717629986741542,
 'recall': 0.9748697061510166,
 'support': 732383}
### report_full
macro
{'f1-score': 0.3199826958221725,
 'precision': 0.3663874186611959,
 'recall': 0.2985710054313986,
 'support': 772066}
micro
{'f1-score': 0.9491554715380848,
 'precision': 0.9748697061510166,
 'recall': 0.9247629088704852,
 'support': 772066}
weighted
{'f1-score': 0.9397425158683885,
 'precision': 0.967042184895364,
 'recall': 0.9247629088704852,
 'support': 772066}
## test
PPCR: 0.942254
### report
macro
{'f1-score': 0.353114623687831,
 'precision': 0.36549274591754954,
 'recall': 0.34411997637939823,
 'support': 189100}
micro
{'f1-score': 0.9736647276573241,
 'precision': 0.9736647276573241,
 'recall': 0.9736647276573241,
 'support': 189100}
weighted
{'f1-score': 0.9716891513173243,
 'precision': 0.9704781365160648,
 'recall': 0.9736647276573241,
 'support': 189100}
### report_full
macro
{'f1-score': 0.31571589786719645,
 'precision': 0.36549274591754954,
 'recall': 0.2924091869748284,
 'support': 200689}
micro
{'f1-score': 0.9447162439165805,
 'precision': 0.9736647276573241,
 'recall': 0.9174394211939867,
 'support': 200689}
weighted
{'f1-score': 0.9352219440797196,
 'precision': 0.9660112737309074,
 'recall': 0.9174394211939867,
 'support': 200689}
```

## javascript
### Summary
94 rules, avg.len. 11.7

| | |
|-|-|
|Min support|92|
|Max support|62842|
|Min confidence|0.922656238079071|
|Max confidence|0.9999613761901855|

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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.997. Support: 27152.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.roles in {ARGUMENT}<br>	∧ -5.label in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.998. Support: 248.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +2.reserved = :<br>⇒ y = ⏎<br>Confidence: 0.938. Support: 235.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -4.diff_line = 0<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +2.reserved not in {:}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 4156.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.996. Support: 3695.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_col ≥ 4<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.965. Support: 1993.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = +<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_col ≤ 3<br>	∧ -4.roles in {RIGHT}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ⏎<br>Confidence: 0.988. Support: 522.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, [}<br>	∧ -2.label in {<newline>} and not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.997. Support: 461.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -2.reserved not in {(}<br>	∧ -2.length ≤ 3<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 36<br>⇒ y = '<br>Confidence: 0.973. Support: 1313.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -2.reserved not in {(}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≤ 35<br>⇒ y = '<br>Confidence: 0.992. Support: 24798.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {FOR} and not in {BODY}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 736.` |
| 12 | `  •••start_col ≥ 11<br>	∧ •••start_line ≤ 46<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -5.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {IDENTIFIER}<br>	∧ +2.length ≤ 2<br>	∧ +3.roles in {LITERAL}<br>	∧ +4.roles not in {CALLEE}<br>	∧ +5.reserved not in {=}<br>	∧ ^1.internal_type = Program<br>	∧ ^1.roles not in {FOR}<br>⇒ y = ⏎<br>Confidence: 0.952. Support: 135.` |
| 13 | `  •••start_col ≥ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {FOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.954. Support: 97.` |
| 14 | `  •••start_col ≥ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -2.reserved not in {'}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {File, IfStatement, Program}<br>	∧ ^1.roles not in {FOR, STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.923. Support: 640.` |
| 15 | `  •••start_col ≤ 10<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length = 0<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {FOR}<br>⇒ y = ⏎<br>Confidence: 0.976. Support: 566.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type = VariableDeclarator<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.996. Support: 409.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ ^1.internal_type = VariableDeclarator<br>⇒ y = ∅<br>Confidence: 0.998. Support: 295.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -2.length ≤ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = VariableDeclarator<br>⇒ y = ∅<br>Confidence: 0.985. Support: 171.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = `<br>	∧ -2.length ≤ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = VariableDeclarator<br>⇒ y = ∅<br>Confidence: 0.995. Support: 103.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, `, {}<br>	∧ -2.length ≤ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = VariableDeclarator<br>⇒ y = ␣<br>Confidence: 0.989. Support: 30184.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -3.label not in {<space>}<br>	∧ -5.diff_offset ≥ 12<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {MAP}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.960. Support: 3173.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.979. Support: 589.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.978. Support: 483.` |
| 24 | `  •••start_col ≤ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -3.label in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {], }}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles in {CALL}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 586.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.roles in {ARGUMENT}<br>	∧ -3.label not in {<newline>}<br>	∧ -4.roles not in {LITERAL}<br>	∧ -5.label in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {], }}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.length ≤ 2<br>⇒ y = ␣<br>Confidence: 0.998. Support: 833.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -3.label not in {<newline>}<br>	∧ -4.roles not in {LITERAL}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {], }}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.length ≤ 2<br>⇒ y = ␣<br>Confidence: 0.970. Support: 15348.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ]<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 753.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {]}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 560.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 298.` |
| 30 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 272.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 132.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -4.diff_offset ≥ 7<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, ), `}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 17137.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -3.length ≥ 3<br>	∧ -4.diff_offset ≤ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, ), `}<br>	∧ ^1.internal_type = BinaryExpression<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 92.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -3.length ≤ 2<br>	∧ -4.diff_offset ≤ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, ), `}<br>	∧ ^1.internal_type = BinaryExpression<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 2008.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -4.diff_offset ≤ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, ), `}<br>	∧ +1.roles in {BINARY}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 288.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, ), `}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 230.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ -3.length ≥ 2<br>	∧ -4.diff_offset ≤ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, ), `}<br>	∧ +1.roles not in {BINARY}<br>	∧ ^1.internal_type not in {BinaryExpression, VariableDeclarator}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 119.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ +2.reserved = }<br>	∧ +5.length ≥ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 126.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -5.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ +2.reserved not in {}}<br>	∧ +3.length ≥ 14<br>	∧ +5.length ≥ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.956. Support: 607.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ +2.reserved = }<br>	∧ +5.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 750.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.966. Support: 11730.` |
| 42 | `  •••start_line ≥ 7<br>	∧ -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.roles in {INCOMPLETE}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎⏎<br>Confidence: 0.991. Support: 291.` |
| 43 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -3.label not in {<space>}<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ +4.roles not in {CALLEE}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.967. Support: 12217.` |
| 44 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 12942.` |
| 45 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 1538.` |
| 46 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, [, const, {, }}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>⇒ y = ␣<br>Confidence: 1.000. Support: 11330.` |
| 47 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.999. Support: 4610.` |
| 48 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +3.length ≥ 5<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.933. Support: 172.` |
| 49 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 4603.` |
| 50 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 3659.` |
| 51 | `  •••start_col ≥ 23<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {STRING} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 830.` |
| 52 | `  •••start_col ≥ 23<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if}<br>	∧ -3.roles in {INCOMPLETE}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 297.` |
| 53 | `  •••start_col ≥ 23<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -5.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ␣<br>Confidence: 0.986. Support: 906.` |
| 54 | `  •••start_col ≥ 23<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if}<br>	∧ -3.reserved = :<br>	∧ -5.diff_offset ≥ 19<br>	∧ -5.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.953. Support: 247.` |
| 55 | `  •••start_col ≥ 23<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ -1.roles in {VALUE}<br>	∧ -2.label in {<space>}<br>	∧ -4.label not in {<space>}<br>	∧ -5.diff_offset ≤ 18<br>	∧ -5.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ␣<br>Confidence: 0.931. Support: 94.` |
| 56 | `  •••start_col ≤ 22<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.930. Support: 2340.` |
| 57 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ -1.length ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {NAME}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1816.` |
| 58 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = let<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1571.` |
| 59 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 1530.` |
| 60 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ><br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 1191.` |
| 61 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -2.label in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = IfStatement<br>⇒ y = ␣<br>Confidence: 0.999. Support: 585.` |
| 62 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, >, const, return, {, }}<br>	∧ -2.label in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≥ 18<br>	∧ ^1.internal_type not in {IfStatement, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 199.` |
| 63 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, >, const, return, {}<br>	∧ -2.label in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 9415.` |
| 64 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ -2.label in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length = 0<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 485.` |
| 65 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = new<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 1012.` |
| 66 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, return, {}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = ConditionalExpression<br>⇒ y = ␣<br>Confidence: 0.951. Support: 644.` |
| 67 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {), ,, ;, const, return, {}<br>	∧ -1.roles in {STATEMENT}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = BlockStatement<br>	∧ ^2.roles in {FUNCTION}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.940. Support: 141.` |
| 68 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {), ,, ;, return, {}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ␣<br>Confidence: 0.955. Support: 1007.` |
| 69 | `  -1.diff_col ≤ 1<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {), ,, :, ;, >, const, return, {}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ∅<br>Confidence: 0.924. Support: 966.` |
| 70 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, >, const, return, {}<br>	∧ -2.label not in {<-space>, <newline>}<br>	∧ -3.reserved not in {}}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ∅<br>Confidence: 0.987. Support: 6926.` |
| 71 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, new, {}<br>	∧ -2.diff_line ≥ 1<br>	∧ -2.label not in {<-space>}<br>	∧ -2.roles in {COMMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = '<br>Confidence: 0.998. Support: 310.` |
| 72 | `  -1.internal_type = DirectiveLiteral<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = '<br>Confidence: 0.998. Support: 323.` |
| 73 | `  -1.internal_type not in {CommentLine, DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, ;, const}<br>	∧ -2.label not in {<-space>}<br>	∧ -5.reserved = !<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.950. Support: 271.` |
| 74 | `  -1.internal_type not in {CommentLine, DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, >, const, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -5.reserved = !<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.internal_type not in {IfStatement, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 283.` |
| 75 | `  -1.internal_type not in {CommentLine, DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = new<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 167.` |
| 76 | `  -1.internal_type not in {CommentLine, DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.length ≥ 4<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {CALLEE}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 133.` |
| 77 | `  -1.internal_type not in {CommentLine, DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, new, {}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {MAP} and not in {CALLEE}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {EXPRESSION} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 154.` |
| 78 | `  -1.internal_type not in {CommentLine, DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {MAP}<br>	∧ ^1.roles not in {EXPRESSION}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.928. Support: 118.` |
| 79 | `  -1.internal_type not in {CommentLine, DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.label in {<newline>} and not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved = .<br>	∧ ^1.roles in {FUNCTION}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 139.` |
| 80 | `  -1.diff_offset ≥ 5<br>	∧ -1.internal_type not in {CommentLine, DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.label not in {<-space>, <newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {MAP}<br>	∧ +2.reserved = .<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 189.` |
| 81 | `  -1.diff_offset ≥ 5<br>	∧ -1.internal_type not in {CommentLine, DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -2.label not in {<-space>, <newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {CALLEE, EXPRESSION, MAP}<br>	∧ +2.reserved = .<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 675.` |
| 82 | `  -1.diff_offset ≤ 4<br>	∧ -1.internal_type not in {CommentLine, DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -2.label not in {<-space>, <newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {CALLEE, MAP}<br>	∧ +2.reserved = .<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 12046.` |
| 83 | `  -1.internal_type not in {CommentLine, DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.reserved not in {const}<br>	∧ -4.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {CALLEE, MAP}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {.}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 1383.` |
| 84 | `  -1.internal_type not in {CommentLine, DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.reserved not in {const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {CALLEE, MAP}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {.}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 1980.` |
| 85 | `  -1.internal_type not in {CommentLine, DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label not in {<-space>, <newline>}<br>	∧ -3.reserved not in {const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {CALLEE, MAP}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = ;<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 9207.` |
| 86 | `  -1.internal_type not in {CommentLine, DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.reserved not in {const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {CALLEE, MAP}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {., ;}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 62842.` |
| 87 | `  -1.diff_col ≥ 8<br>	∧ -1.internal_type not in {CommentLine, DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.reserved not in {const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {CALLEE, MAP}<br>	∧ +2.reserved not in {.}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 1421.` |
| 88 | `  -1.diff_col ≤ 7<br>	∧ -1.internal_type not in {CommentLine, DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≥ 6<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {.}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 1283.` |
| 89 | `  •••start_col ≥ 14<br>	∧ -1.diff_col ≤ 7<br>	∧ -1.internal_type not in {CommentLine, DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≤ 5<br>	∧ -2.label not in {<-space>}<br>	∧ -3.reserved not in {const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {CALLEE, MAP}<br>	∧ +2.reserved not in {.}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 161.` |
| 90 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {CommentLine, DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, new, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.reserved not in {const}<br>	∧ -4.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {CALLEE, MAP}<br>	∧ +2.reserved not in {.}<br>	∧ +2.length ≤ 32<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {FOR} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 588.` |
| 91 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {CommentLine, DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, >, const, new, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.reserved not in {const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {CALLEE, MAP}<br>	∧ +2.reserved not in {.}<br>	∧ +2.length ≤ 32<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 10243.` |
| 92 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -5.diff_offset ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {NAME}<br>	∧ +2.roles in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 736.` |
| 93 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -5.diff_offset ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {NAME}<br>	∧ +1.length ≥ 13<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 170.` |
| 94 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, if, {}<br>	∧ -5.diff_offset ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 13<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.989. Support: 2275.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 11.73404255319149, "max_conf": 0.9999613761901855, "max_support": 62842, "min_conf": 0.922656238079071, "min_support": 92, "num_rules": 94}}
```
</details>
