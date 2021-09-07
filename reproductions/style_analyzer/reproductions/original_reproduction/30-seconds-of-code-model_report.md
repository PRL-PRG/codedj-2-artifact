# Model report for file:///tmp/top-repos-quality-repos-2279v_7n/30-seconds-of-code HEAD 3a122c9cfcbdc091227879a06a32bc67ccd0d35d

### Dump

```json
{'created_at': '2021-08-13 16:29:44',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-80-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.9 (default, Jan 26 2021, 15:33:00) [GCC 8.4.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '17.3 kB',
 'tags': [],
 'uuid': '3e04e1ef-7ae9-4c7a-bf77-0682affc6e56',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-2279v_7n/30-seconds-of-code 3a122c9cfcbdc091227879a06a32bc67ccd0d35d

# javascript
82 rules, avg.len. 9.0
## train
PPCR: 1.000000
### report
macro
{'f1-score': 0.9109501687776287,
 'precision': 0.9571317043937887,
 'recall': 0.8728626201510359,
 'support': 44039}
micro
{'f1-score': 0.9585594586616408,
 'precision': 0.9585594586616408,
 'recall': 0.9585594586616408,
 'support': 44039}
weighted
{'f1-score': 0.957686476478527,
 'precision': 0.9585696770593655,
 'recall': 0.9585594586616408,
 'support': 44039}
### report_full
macro
{'f1-score': 0.9109501687776287,
 'precision': 0.9571317043937887,
 'recall': 0.8728626201510359,
 'support': 44039}
micro
{'f1-score': 0.9585594586616408,
 'precision': 0.9585594586616408,
 'recall': 0.9585594586616408,
 'support': 44039}
weighted
{'f1-score': 0.957686476478527,
 'precision': 0.9585696770593655,
 'recall': 0.9585594586616408,
 'support': 44039}
## test
PPCR: 1.000000
### report
macro
{'f1-score': 0.9455896951699992,
 'precision': 0.9676005597097526,
 'recall': 0.9255606263204481,
 'support': 11493}
micro
{'f1-score': 0.9729400504655007,
 'precision': 0.9729400504655007,
 'recall': 0.9729400504655007,
 'support': 11493}
weighted
{'f1-score': 0.9727326826652207,
 'precision': 0.9728799927331918,
 'recall': 0.9729400504655007,
 'support': 11493}
### report_full
macro
{'f1-score': 0.9455896951699992,
 'precision': 0.9676005597097526,
 'recall': 0.9255606263204481,
 'support': 11493}
micro
{'f1-score': 0.9729400504655007,
 'precision': 0.9729400504655007,
 'recall': 0.9729400504655007,
 'support': 11493}
weighted
{'f1-score': 0.9727326826652207,
 'precision': 0.9728799927331918,
 'recall': 0.9729400504655007,
 'support': 11493}
```

## javascript
### Summary
82 rules, avg.len. 9.0

| | |
|-|-|
|Min support|139|
|Max support|18430|
|Min confidence|0.8042596578598022|
|Max confidence|0.9997513890266418|

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
               'min_samples_leaf_max': 130,
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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 1.000. Support: 2011.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.964. Support: 235.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles in {CALL}<br>⇒ y = '<br>Confidence: 0.983. Support: 1810.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {CALL}<br>⇒ y = '<br>Confidence: 0.998. Support: 229.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ␣<br>Confidence: 0.827. Support: 211.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.999. Support: 859.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -4.length ≥ 9<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>⇒ y = ⏎⏎<br>Confidence: 0.884. Support: 246.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -4.length ≤ 8<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>⇒ y = ⏎<br>Confidence: 0.934. Support: 1383.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 2049.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1691.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>⇒ y = ␣<br>Confidence: 0.934. Support: 1282.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -3.reserved = ><br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles in {CALL}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.999. Support: 769.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {CALL}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 174.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {CALL}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 3369.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +4.roles in {CALLEE}<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 218.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.label in {<space>}<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +4.roles not in {CALLEE}<br>	∧ ^1.roles in {EXPRESSION} and not in {CALL}<br>⇒ y = ␣<br>Confidence: 0.889. Support: 203.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.label not in {<space>}<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +4.roles not in {CALLEE}<br>	∧ ^1.roles in {EXPRESSION} and not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.837. Support: 242.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +4.roles not in {CALLEE}<br>	∧ ^1.roles not in {CALL, EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.890. Support: 1526.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.label in {<space>}<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.941. Support: 179.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 15689.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -3.reserved not in {>}<br>	∧ -4.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.internal_type = VariableDeclarator<br>⇒ y = ∅<br>Confidence: 0.997. Support: 187.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = VariableDeclarator<br>⇒ y = ␣<br>Confidence: 0.951. Support: 751.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = const<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 322.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, const}<br>	∧ -2.roles in {MAP}<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.946. Support: 341.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, const}<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {KEY}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 196.` |
| 26 | `  •••start_line ≤ 30<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, const}<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ␣<br>Confidence: 0.903. Support: 159.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {Identifier, StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {FUNCTION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.894. Support: 606.` |
| 28 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {FUNCTION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.856. Support: 226.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {File, VariableDeclarator}<br>	∧ ^1.roles in {STATEMENT} and not in {FUNCTION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.886. Support: 293.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File, VariableDeclarator}<br>	∧ ^1.roles not in {FUNCTION, OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 18207.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles in {MAP}<br>	∧ +4.roles in {CALL}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 205.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ +4.roles not in {CALL}<br>⇒ y = ␣<br>Confidence: 0.879. Support: 195.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {MAP}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.998. Support: 905.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.957. Support: 741.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ∅<br>Confidence: 0.970. Support: 216.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 355.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, const, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.804. Support: 493.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, const, {}<br>	∧ -2.roles in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 342.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, const, {}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ␣<br>Confidence: 0.917. Support: 139.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ ^1.roles in {FUNCTION} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.911. Support: 559.` |
| 41 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.roles in {FILE} and not in {FUNCTION, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.907. Support: 210.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.roles not in {FILE, FUNCTION, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 18430.` |
| 43 | `  •••start_line ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>⇒ y = ␣<br>Confidence: 0.909. Support: 204.` |
| 44 | `  •••start_line ≤ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles in {MAP}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 192.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -4.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.internal_type = VariableDeclarator<br>⇒ y = ∅<br>Confidence: 0.998. Support: 247.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = VariableDeclarator<br>⇒ y = ␣<br>Confidence: 0.950. Support: 705.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const}<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.812. Support: 152.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ -2.label in {<space>}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 258.` |
| 49 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {FILE} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 239.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ -2.roles not in {MAP}<br>	∧ -5.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {STATEMENT} and not in {DECLARATION, FILE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.891. Support: 410.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {DECLARATION, FILE, OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 18013.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -4.length ≥ 8<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>⇒ y = ⏎⏎<br>Confidence: 0.853. Support: 214.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -4.length ≤ 7<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>⇒ y = ⏎<br>Confidence: 0.938. Support: 1322.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -5.label in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.internal_type = VariableDeclarator<br>⇒ y = ∅<br>Confidence: 0.998. Support: 205.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = VariableDeclarator<br>⇒ y = ␣<br>Confidence: 0.947. Support: 747.` |
| 56 | `  •••start_line ≤ 27<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, const, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ␣<br>Confidence: 0.842. Support: 149.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.diff_offset ≤ 13<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {FILE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.855. Support: 244.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {FUNCTION} and not in {FILE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.915. Support: 617.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {FILE, FUNCTION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 18381.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -3.reserved = ><br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +2.reserved = (<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.998. Support: 812.` |
| 61 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -3.reserved not in {>}<br>	∧ -4.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ∅<br>Confidence: 0.998. Support: 215.` |
| 62 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.932. Support: 747.` |
| 63 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, const}<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.808. Support: 456.` |
| 64 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 276.` |
| 65 | `  •••start_line ≤ 28<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, const}<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ␣<br>Confidence: 0.878. Support: 185.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const}<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {STATEMENT} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.881. Support: 459.` |
| 67 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const}<br>	∧ -3.diff_offset ≤ 13<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.843. Support: 297.` |
| 68 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const}<br>	∧ -2.label in {<space>}<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR, STATEMENT}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 219.` |
| 69 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const}<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, STATEMENT}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 18159.` |
| 70 | `  -1.reserved not in {,, ;}<br>	∧ -3.reserved = ><br>	∧ +1.internal_type = Identifier<br>	∧ +1.reserved not in {=, {}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.993. Support: 821.` |
| 71 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +3.reserved = =<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 216.` |
| 72 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.roles in {EXPRESSION}<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +3.reserved not in {=}<br>	∧ ^1.roles in {EXPRESSION} and not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 153.` |
| 73 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +3.reserved not in {=}<br>	∧ ^1.roles not in {CALL, EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.891. Support: 1522.` |
| 74 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.label not in {<space>}<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.828. Support: 422.` |
| 75 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -3.reserved = ><br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles in {CALLEE}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.997. Support: 753.` |
| 76 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.roles in {EXPRESSION}<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {ARGUMENT, CALL}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 296.` |
| 77 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles in {EXPRESSION} and not in {IDENTIFIER}<br>	∧ ^1.roles in {ARGUMENT, CALL}<br>⇒ y = ∅<br>Confidence: 0.867. Support: 184.` |
| 78 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {CALL} and not in {ARGUMENT}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 2716.` |
| 79 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.label in {<space>}<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +3.reserved not in {=}<br>	∧ ^1.roles in {EXPRESSION} and not in {CALL}<br>⇒ y = ␣<br>Confidence: 0.842. Support: 225.` |
| 80 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.label not in {<space>}<br>	∧ -3.reserved not in {>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +3.reserved not in {=}<br>	∧ ^1.roles in {EXPRESSION} and not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.829. Support: 237.` |
| 81 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -3.reserved not in {>}<br>	∧ -5.diff_offset ≥ 9<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.835. Support: 379.` |
| 82 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -3.reserved not in {>}<br>	∧ -5.diff_offset ≤ 8<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.894. Support: 193.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.951219512195122, "max_conf": 0.9997513890266418, "max_support": 18430, "min_conf": 0.8042596578598022, "min_support": 139, "num_rules": 82}}
```
</details>
