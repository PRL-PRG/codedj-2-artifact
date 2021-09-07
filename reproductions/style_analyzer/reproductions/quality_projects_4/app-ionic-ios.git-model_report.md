# Model report for file:///tmp/top-repos-quality-repos-sdjea3s4/app-ionic-ios.git HEAD e702dd34b3c1ddf728ce67ed7662679fed3dacc6

### Dump

```json
{'created_at': '2021-09-02 08:41:51',
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
 'size': '24.3 kB',
 'tags': [],
 'uuid': '4c8aa4df-ab87-4cfb-b644-436591cf905b',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-sdjea3s4/app-ionic-ios.git e702dd34b3c1ddf728ce67ed7662679fed3dacc6

# javascript
196 rules, avg.len. 9.3
## train
PPCR: 0.892651
### report
macro
{'f1-score': 0.3188466983749642,
 'precision': 0.367631665300594,
 'recall': 0.29861272798724176,
 'support': 30900}
micro
{'f1-score': 0.8694174757281553,
 'precision': 0.8694174757281553,
 'recall': 0.8694174757281553,
 'support': 30900}
weighted
{'f1-score': 0.8405580705113036,
 'precision': 0.8263342876349277,
 'recall': 0.8694174757281553,
 'support': 30900}
### report_full
macro
{'f1-score': 0.278349854101279,
 'precision': 0.367631665300594,
 'recall': 0.25156886861946065,
 'support': 34616}
micro
{'f1-score': 0.8201050125160266,
 'precision': 0.8694174757281553,
 'recall': 0.7760862029119482,
 'support': 34616}
weighted
{'f1-score': 0.7617449800135725,
 'precision': 0.7762344545891329,
 'recall': 0.7760862029119482,
 'support': 34616}
## test
PPCR: 0.921311
### report
macro
{'f1-score': 0.3025635663806547,
 'precision': 0.3655977956563339,
 'recall': 0.28620281288220467,
 'support': 8266}
micro
{'f1-score': 0.8727316719090249,
 'precision': 0.8727316719090249,
 'recall': 0.8727316719090249,
 'support': 8266}
weighted
{'f1-score': 0.8429741329472293,
 'precision': 0.8423683033972638,
 'recall': 0.8727316719090249,
 'support': 8266}
### report_full
macro
{'f1-score': 0.2689068130589904,
 'precision': 0.3655977956563339,
 'recall': 0.24894003716883176,
 'support': 8972}
micro
{'f1-score': 0.8369880496577329,
 'precision': 0.8727316719090249,
 'recall': 0.8040570664288899,
 'support': 8972}
weighted
{'f1-score': 0.7846136255333261,
 'precision': 0.8062136317025062,
 'recall': 0.8040570664288899,
 'support': 8972}
```

## javascript
### Summary
113 rules, avg.len. 8.5

| | |
|-|-|
|Min support|146|
|Max support|6237|
|Min confidence|0.9210749268531799|
|Max confidence|0.9997966885566711|

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
                     'min_samples_leaf': 93,
                     'min_samples_split': 194,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.reserved = .<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 2391.` |
| 2 | `  -1.reserved not in {.}<br>	∧ -4.reserved = '<br>	∧ +1.roles in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 224.` |
| 3 | `  -1.reserved not in {.}<br>	∧ -4.reserved not in {'}<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ +1.roles in {LITERAL}<br>	∧ ^1.roles in {LIST} and not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.975. Support: 218.` |
| 4 | `  -1.reserved not in {.}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 301.` |
| 5 | `  -1.reserved not in {.}<br>	∧ -1.roles in {EXPRESSION, IDENTIFIER}<br>	∧ -4.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 6237.` |
| 6 | `  -1.reserved = )<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type = Identifier<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 219.` |
| 7 | `  -1.reserved = )<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.length ≥ 3<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {CALL, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 250.` |
| 8 | `  -1.reserved = )<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.length ≤ 2<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 737.` |
| 9 | `  -1.diff_offset ≥ 3<br>	∧ -1.reserved not in {), .}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {CALL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 355.` |
| 10 | `  -1.diff_offset ≥ 3<br>	∧ -1.reserved not in {), .}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {EXPRESSION, LITERAL}<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles not in {CALL, OPERATOR}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 155.` |
| 11 | `  -1.diff_offset ≤ 2<br>	∧ -1.reserved not in {), .}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ -2.length ≤ 22<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {MAP} and not in {CALL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 154.` |
| 12 | `  -1.diff_offset ≤ 2<br>	∧ -1.reserved not in {), ., ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {ARGUMENT, LITERAL}<br>	∧ ^1.roles not in {CALL, MAP, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 172.` |
| 13 | `  -1.reserved not in {), ., ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≤ 2<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 218.` |
| 14 | `  -1.roles in {EXPRESSION}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.998. Support: 303.` |
| 15 | `  -1.internal_type = Identifier<br>	∧ -1.roles in {CALL, EXPRESSION}<br>	∧ -4.roles not in {LITERAL}<br>	∧ +2.roles not in {RIGHT}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 776.` |
| 16 | `  -1.diff_col ≤ 2<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.length ≤ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = ,<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 211.` |
| 17 | `  -1.diff_col ≤ 2<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved not in {,}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.938. Support: 3123.` |
| 18 | `  -1.diff_col ≤ 2<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved not in {,}<br>	∧ ^1.internal_type = FunctionDeclaration<br>⇒ y = ␣<br>Confidence: 0.923. Support: 291.` |
| 19 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +2.reserved not in {,}<br>	∧ +3.internal_type = Identifier<br>	∧ ^1.internal_type not in {FunctionDeclaration, MemberExpression}<br>	∧ ^1.roles not in {BINARY}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 402.` |
| 20 | `  -1.diff_col ≤ 2<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ +2.reserved not in {,}<br>	∧ +3.internal_type not in {Identifier}<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {BINARY}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 509.` |
| 21 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved = )<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ -4.reserved not in {'}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +2.reserved not in {,}<br>	∧ +3.internal_type not in {Identifier}<br>	∧ ^1.internal_type not in {BlockStatement, FunctionDeclaration, MemberExpression}<br>	∧ ^1.roles not in {BINARY, EXPRESSION}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 420.` |
| 22 | `  +2.internal_type = Identifier<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.993. Support: 213.` |
| 23 | `  -3.length ≥ 2<br>	∧ +2.internal_type not in {Identifier}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.993. Support: 371.` |
| 24 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {IDENTIFIER}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2318.` |
| 25 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 857.` |
| 26 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 3849.` |
| 27 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {Identifier}<br>	∧ -1.reserved = .<br>	∧ -1.roles not in {LITERAL}<br>	∧ +1.reserved not in {}}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2459.` |
| 28 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {Identifier}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {LITERAL}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {MAP}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 447.` |
| 29 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {.}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.roles in {ARGUMENT}<br>	∧ -5.diff_offset ≥ 8<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {MAP}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 257.` |
| 30 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {Identifier}<br>	∧ -1.reserved = )<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {BLOCK} and not in {MAP}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 376.` |
| 31 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {), ,, .}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles in {BLOCK} and not in {MAP}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 146.` |
| 32 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {,, ., {}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +3.roles in {FUNCTION}<br>	∧ ^1.roles not in {BLOCK, MAP}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 178.` |
| 33 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {Identifier}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -3.label not in {<space>}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {ARGUMENT}<br>	∧ +3.roles not in {FUNCTION}<br>	∧ +5.roles not in {BINARY}<br>	∧ ^1.roles not in {BLOCK, MAP}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 247.` |
| 34 | `  -1.roles in {IDENTIFIER}<br>	∧ ^2.roles in {EXPRESSION} and not in {BLOCK}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 4301.` |
| 35 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = =<br>	∧ ^2.roles not in {BLOCK, EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 709.` |
| 36 | `  -1.roles not in {IDENTIFIER}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 2637.` |
| 37 | `  -1.reserved = =<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +1.reserved not in {,}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 788.` |
| 38 | `  -1.reserved not in {=}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {,, }}<br>	∧ +1.roles in {EXPRESSION, IDENTIFIER, VALUE}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {)}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 155.` |
| 39 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.921. Support: 5898.` |
| 40 | `  -1.length ≥ 3<br>	∧ +1.roles in {IDENTIFIER}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 735.` |
| 41 | `  •••start_col ≥ 17<br>	∧ -1.reserved = ;<br>	∧ -1.length ≤ 2<br>	∧ +1.roles in {IDENTIFIER}<br>	∧ +4.reserved = (<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.932. Support: 168.` |
| 42 | `  -1.reserved = =<br>	∧ -1.length ≤ 2<br>	∧ +1.roles in {IDENTIFIER}<br>	∧ +4.reserved not in {(}<br>	∧ ^1.roles not in {BODY, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 315.` |
| 43 | `  -1.reserved = (<br>	∧ -1.length ≤ 2<br>	∧ +1.roles in {IDENTIFIER}<br>	∧ +4.reserved not in {(}<br>	∧ ^1.roles not in {BODY, IDENTIFIER, INITIALIZATION, MAP, RIGHT}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 668.` |
| 44 | `  -1.reserved = =<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ +3.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 388.` |
| 45 | `  -1.reserved not in {=}<br>	∧ -1.roles in {IDENTIFIER} and not in {STRING}<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ +3.roles not in {STRING}<br>	∧ ^1.roles in {VARIABLE} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 305.` |
| 46 | `  -1.reserved not in {=}<br>	∧ -1.roles in {CALL, IDENTIFIER} and not in {STRING}<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ +3.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 762.` |
| 47 | `  -1.diff_offset ≥ 2<br>	∧ -1.reserved not in {=}<br>	∧ -1.roles in {IDENTIFIER} and not in {CALL, STRING}<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ +3.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER, VARIABLE}<br>	∧ ^2.roles in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 570.` |
| 48 | `  -1.diff_offset ≥ 2<br>	∧ -1.reserved not in {=}<br>	∧ -1.roles in {IDENTIFIER} and not in {CALL, STRING}<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ +3.roles not in {STRING}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 584.` |
| 49 | `  -1.diff_offset ≥ 2<br>	∧ -1.reserved not in {=}<br>	∧ -1.roles in {IDENTIFIER} and not in {CALL, STRING}<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ +3.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR, VARIABLE}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 2598.` |
| 50 | `  -1.reserved not in {;, =, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {RIGHT} and not in {IDENTIFIER}<br>	∧ +3.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER, SCOPE}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 167.` |
| 51 | `  -1.reserved not in {;, =, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.roles not in {FUNCTION}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {IDENTIFIER, RIGHT}<br>	∧ +3.roles not in {STRING}<br>	∧ ^1.roles not in {FILE, IDENTIFIER, SCOPE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 518.` |
| 52 | `  -1.reserved not in {;, =, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.roles not in {FUNCTION}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {IDENTIFIER, RIGHT}<br>	∧ +3.roles not in {STRING}<br>	∧ +4.roles not in {STRING}<br>	∧ ^1.roles not in {FILE, IDENTIFIER, SCOPE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 182.` |
| 53 | `  -1.roles in {EXPRESSION} and not in {LITERAL}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 2400.` |
| 54 | `  -1.roles in {EXPRESSION} and not in {LITERAL}<br>	∧ ^1.roles in {MAP} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 568.` |
| 55 | `  -1.roles in {EXPRESSION} and not in {LITERAL}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {MAP, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 1621.` |
| 56 | `  -1.roles in {EXPRESSION} and not in {LITERAL}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {MAP, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 698.` |
| 57 | `  -1.roles in {EXPRESSION} and not in {BINARY, LITERAL}<br>	∧ +1.reserved not in {)}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {MAP, QUALIFIED}<br>	∧ ^2.roles in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.949. Support: 326.` |
| 58 | `  -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.931. Support: 3282.` |
| 59 | `  -1.roles not in {EXPRESSION}<br>	∧ -3.roles in {LITERAL}<br>	∧ -4.reserved = '<br>	∧ -4.roles not in {IDENTIFIER}<br>	∧ +1.roles in {STRING} and not in {VALUE}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.967. Support: 230.` |
| 60 | `  -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {RIGHT} and not in {STRING, VALUE}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 382.` |
| 61 | `  -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {RIGHT, STRING, VALUE}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 703.` |
| 62 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.reserved = var<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {RIGHT, STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 304.` |
| 63 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.length ≤ 35<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ -5.roles in {FUNCTION}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {MAP}<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ␣<br>Confidence: 0.925. Support: 260.` |
| 64 | `  -1.reserved not in {;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.length ≤ 35<br>	∧ -5.reserved not in {var}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {RIGHT, STRING, VALUE}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {MAP}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1094.` |
| 65 | `  -1.reserved not in {;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {ARGUMENT}<br>	∧ -2.length ≤ 35<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {BINARY} and not in {MAP}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 349.` |
| 66 | `  -1.reserved = var<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.length ≤ 35<br>	∧ -3.roles not in {EXPRESSION, IDENTIFIER}<br>	∧ -5.diff_col ≤ 61<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {STRING}<br>	∧ +3.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {BINARY, MAP}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 161.` |
| 67 | `  -1.internal_type = StringLiteral<br>	∧ -4.diff_offset ≤ 28<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ ^1.roles in {LIST}<br>⇒ y = '<br>Confidence: 0.998. Support: 229.` |
| 68 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {STRING}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 1932.` |
| 69 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 5347.` |
| 70 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {CALL}<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {AssignmentExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 522.` |
| 71 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {CALL}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {COMMENT}<br>	∧ +4.reserved not in {)}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.927. Support: 1932.` |
| 72 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {CALL}<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {AssignmentExpression, CallExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 581.` |
| 73 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles in {EXPRESSION} and not in {CALL}<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type = BinaryExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 202.` |
| 74 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles in {EXPRESSION} and not in {CALL}<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -2.length ≥ 10<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {AssignmentExpression, BinaryExpression, CallExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 271.` |
| 75 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles in {EXPRESSION} and not in {CALL}<br>	∧ -2.reserved = .<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -2.length ≤ 9<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {AssignmentExpression, BinaryExpression, CallExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 338.` |
| 76 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles in {EXPRESSION} and not in {CALL}<br>	∧ -2.reserved not in {), .}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -2.length ≤ 9<br>	∧ -4.roles in {EXPRESSION}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {AssignmentExpression, BinaryExpression, CallExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 424.` |
| 77 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {CALL, EXPRESSION}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {AssignmentExpression, CallExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 940.` |
| 78 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = var<br>	∧ -1.roles not in {CALL, EXPRESSION}<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +2.roles not in {COMMENT}<br>	∧ +4.reserved not in {=}<br>	∧ +5.roles not in {CALL}<br>	∧ ^1.internal_type not in {AssignmentExpression, CallExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 322.` |
| 79 | `  -1.reserved = ,<br>	∧ -5.roles in {ARGUMENT}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 386.` |
| 80 | `  -1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.933. Support: 812.` |
| 81 | `  -1.reserved not in {,, =}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -5.diff_offset ≥ 22<br>	∧ +1.roles not in {STRING}<br>	∧ +2.reserved not in {,}<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles in {CALL}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 3076.` |
| 82 | `  -1.reserved not in {,, =}<br>	∧ -1.roles in {ARGUMENT} and not in {LITERAL}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -5.diff_offset ≤ 21<br>	∧ +1.roles not in {STRING}<br>	∧ +2.reserved not in {,}<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles in {CALL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 463.` |
| 83 | `  -1.reserved not in {,, =}<br>	∧ +2.reserved not in {,}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 2506.` |
| 84 | `  -1.diff_col ≤ 21<br>	∧ -1.reserved not in {,, =}<br>	∧ -2.reserved not in {)}<br>	∧ +1.reserved = ;<br>	∧ +2.reserved not in {,, =}<br>	∧ ^1.internal_type not in {BinaryExpression, MemberExpression}<br>	∧ ^1.roles in {STATEMENT} and not in {CALL}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 754.` |
| 85 | `  -1.diff_col ≤ 21<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -1.length ≤ 1<br>	∧ -2.reserved not in {)}<br>	∧ -3.reserved not in {)}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.internal_type not in {BinaryExpression, MemberExpression}<br>	∧ ^1.roles in {STATEMENT} and not in {CALL}<br>	∧ ^2.roles in {STATEMENT} and not in {BODY}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 174.` |
| 86 | `  -1.reserved not in {,, =}<br>	∧ -1.roles in {KEY} and not in {LITERAL}<br>	∧ -2.reserved not in {)}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +2.reserved not in {,}<br>	∧ +5.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {ArrayExpression, BinaryExpression, MemberExpression}<br>	∧ ^1.roles not in {CALL}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 429.` |
| 87 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {'}<br>	∧ +1.roles in {STRING}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 225.` |
| 88 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles in {RIGHT}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 527.` |
| 89 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.reserved = .<br>	∧ -3.diff_offset ≥ 4<br>	∧ -4.label not in {<newline>}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {EXPRESSION, STRING}<br>	∧ +2.roles not in {RIGHT}<br>	∧ ^2.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 234.` |
| 90 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {BINARY, STRING}<br>	∧ +2.reserved = }<br>	∧ +2.roles not in {RIGHT}<br>	∧ ^2.roles in {STATEMENT} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.943. Support: 253.` |
| 91 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = =<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {BINARY, STRING}<br>	∧ +2.reserved not in {}}<br>	∧ ^2.roles in {STATEMENT} and not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 257.` |
| 92 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {;, =}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {BINARY, STRING}<br>	∧ +2.reserved not in {}}<br>	∧ +2.roles not in {RIGHT}<br>	∧ ^2.roles in {STATEMENT} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 509.` |
| 93 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {;, =}<br>	∧ -2.label in {'}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {BINARY, STRING}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {}}<br>	∧ ^2.roles in {STATEMENT} and not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.946. Support: 177.` |
| 94 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {;, =}<br>	∧ -1.length ≥ 2<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.reserved not in {), {}<br>	∧ +1.roles not in {BINARY, STRING}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {}}<br>	∧ ^2.roles in {STATEMENT} and not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.948. Support: 262.` |
| 95 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {;, =}<br>	∧ -1.length ≤ 1<br>	∧ -2.label not in {'}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -4.diff_col ≤ 17<br>	∧ -4.length ≥ 6<br>	∧ +1.reserved not in {), {}<br>	∧ +1.roles not in {BINARY, STRING}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {}}<br>	∧ +2.roles not in {RIGHT}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles in {STATEMENT} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 176.` |
| 96 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {;, =}<br>	∧ -1.length ≤ 1<br>	∧ -2.label not in {'}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -4.diff_col ≤ 17<br>	∧ -4.length ≤ 5<br>	∧ +1.reserved not in {), {}<br>	∧ +1.roles not in {BINARY, STRING}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = )<br>	∧ +2.roles not in {RIGHT}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles in {STATEMENT} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 231.` |
| 97 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -4.reserved not in {'}<br>	∧ -5.diff_line ≤ 3<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {BINARY, STRING}<br>	∧ +2.roles not in {RIGHT}<br>	∧ +4.reserved not in {function}<br>	∧ ^1.roles in {IDENTIFIER}<br>	∧ ^2.roles not in {DECLARATION, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 4590.` |
| 98 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -4.reserved not in {'}<br>	∧ -5.diff_line ≤ 3<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {BINARY, STRING}<br>	∧ +2.roles not in {RIGHT}<br>	∧ +3.reserved not in {;}<br>	∧ +4.reserved not in {function}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {DECLARATION, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.937. Support: 1082.` |
| 99 | `  -1.internal_type = Identifier<br>	∧ -1.roles not in {STRING}<br>	∧ ^2.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.935. Support: 809.` |
| 100 | `  -1.internal_type = Identifier<br>	∧ -1.roles not in {STRING}<br>	∧ -3.length ≥ 3<br>	∧ -4.label in {<newline>}<br>	∧ ^1.roles in {CALL}<br>	∧ ^2.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 533.` |
| 101 | `  -1.internal_type = Identifier<br>	∧ -1.roles not in {STRING}<br>	∧ -3.length ≥ 3<br>	∧ -4.label not in {<newline>}<br>	∧ +2.length ≥ 13<br>	∧ ^2.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 162.` |
| 102 | `  -1.diff_col ≥ 5<br>	∧ -1.internal_type = Identifier<br>	∧ -1.roles not in {STRING}<br>	∧ -3.length ≥ 3<br>	∧ -4.label not in {<newline>}<br>	∧ +2.length ≤ 12<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 438.` |
| 103 | `  -1.diff_col ≤ 4<br>	∧ -1.internal_type = Identifier<br>	∧ -1.roles not in {STRING}<br>	∧ -3.length ≥ 3<br>	∧ -4.label not in {<newline>}<br>	∧ +2.length ≤ 12<br>	∧ ^2.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 173.` |
| 104 | `  -1.internal_type = Identifier<br>	∧ -1.roles not in {STRING}<br>	∧ -3.length ≥ 3<br>	∧ -4.label not in {<newline>}<br>	∧ +2.length ≤ 2<br>	∧ ^2.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 742.` |
| 105 | `  -1.internal_type = Identifier<br>	∧ -1.roles not in {STRING}<br>	∧ -3.length ≤ 2<br>	∧ ^1.internal_type = BinaryExpression<br>	∧ ^2.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 191.` |
| 106 | `  -1.internal_type = Identifier<br>	∧ -1.roles not in {STRING}<br>	∧ -3.length ≤ 2<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^2.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.932. Support: 3480.` |
| 107 | `  -1.internal_type not in {Identifier}<br>	∧ -1.roles not in {STRING}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 3285.` |
| 108 | `  -1.internal_type not in {Identifier}<br>	∧ -1.roles not in {STRING}<br>	∧ -4.label in {'}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles in {EXPRESSION} and not in {BODY, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 246.` |
| 109 | `  -1.internal_type not in {Identifier}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type = Identifier<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {BODY, EXPRESSION, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 201.` |
| 110 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {STRING}<br>	∧ -3.internal_type not in {Identifier}<br>	∧ -4.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {BODY, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 456.` |
| 111 | `  •••start_col ≥ 8<br>	∧ -1.internal_type not in {Identifier}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ -3.internal_type not in {Identifier}<br>	∧ -3.reserved not in {.}<br>	∧ -4.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 3<br>	∧ ^1.roles not in {BODY, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 453.` |
| 112 | `  •••start_col ≥ 8<br>	∧ -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {LITERAL} and not in {STRING}<br>	∧ -3.internal_type not in {Identifier}<br>	∧ -3.reserved not in {.}<br>	∧ -4.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≤ 2<br>	∧ ^1.roles not in {BODY, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 299.` |
| 113 | `  •••start_col ≥ 8<br>	∧ -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.diff_col ≤ 11<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -3.internal_type not in {Identifier}<br>	∧ -3.reserved not in {.}<br>	∧ -4.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≤ 2<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ ^1.roles not in {BODY, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 189.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.530973451327434, "max_conf": 0.9997966885566711, "max_support": 6237, "min_conf": 0.9210749268531799, "min_support": 146, "num_rules": 113}}
```
</details>
