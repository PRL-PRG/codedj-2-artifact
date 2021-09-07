# Model report for file:///tmp/top-repos-quality-repos-rbtp_25e/jquery.git HEAD 2f8f39e457c32c454c50545b0fdaa1d7a4a2f8bd

### Dump

```json
{'created_at': '2021-09-01 19:42:04',
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
 'size': '20.9 kB',
 'tags': [],
 'uuid': 'b7178fcf-334e-4967-a732-053f41772edd',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-rbtp_25e/jquery.git 2f8f39e457c32c454c50545b0fdaa1d7a4a2f8bd

# javascript
51 rules, avg.len. 10.9
## train
PPCR: 0.948521
### report
macro
{'f1-score': 0.6327329860554148,
 'precision': 0.6747240690356636,
 'recall': 0.6060003289337668,
 'support': 196231}
micro
{'f1-score': 0.9728177505083294,
 'precision': 0.9728177505083294,
 'recall': 0.9728177505083294,
 'support': 196231}
weighted
{'f1-score': 0.9701121071760799,
 'precision': 0.9693652125857581,
 'recall': 0.9728177505083294,
 'support': 196231}
### report_full
macro
{'f1-score': 0.5673344329501572,
 'precision': 0.6747240690356636,
 'recall': 0.515186248586719,
 'support': 206881}
micro
{'f1-score': 0.9471164341423722,
 'precision': 0.9728177505083294,
 'recall': 0.9227381924874687,
 'support': 206881}
weighted
{'f1-score': 0.9343283775354065,
 'precision': 0.9648615701533193,
 'recall': 0.9227381924874687,
 'support': 206881}
## test
PPCR: 0.950419
### report
macro
{'f1-score': 0.5926786131063142,
 'precision': 0.6760318067099459,
 'recall': 0.566226499667414,
 'support': 44338}
micro
{'f1-score': 0.9627633181469619,
 'precision': 0.9627633181469619,
 'recall': 0.9627633181469619,
 'support': 44338}
weighted
{'f1-score': 0.9573904075576947,
 'precision': 0.961799338772651,
 'recall': 0.9627633181469619,
 'support': 44338}
### report_full
macro
{'f1-score': 0.5293296166387497,
 'precision': 0.6760318067099459,
 'recall': 0.476560779896765,
 'support': 46651}
micro
{'f1-score': 0.9382892437547395,
 'precision': 0.9627633181469619,
 'recall': 0.9150286167499089,
 'support': 46651}
weighted
{'f1-score': 0.9218980831969212,
 'precision': 0.9572192694204139,
 'recall': 0.9150286167499089,
 'support': 46651}
```

## javascript
### Summary
37 rules, avg.len. 10.6

| | |
|-|-|
|Min support|96|
|Max support|26985|
|Min confidence|0.9244604110717773|
|Max confidence|0.9999611377716064|

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
                     'min_samples_leaf': 91,
                     'min_samples_split': 238,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 1.000. Support: 12558.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>⇒ y = "<br>Confidence: 1.000. Support: 11769.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 4287.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved = {<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 2254.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved not in {{}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 1190.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.internal_type = CommentLine<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 757.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {CommentLine}<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.927. Support: 484.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {CommentLine}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 299.` |
| 9 | `  •••start_col ≥ 18<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -2.label not in {<space>}<br>	∧ -4.roles not in {NUMBER}<br>	∧ +1.internal_type not in {CommentLine}<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles in {EXPRESSION}<br>	∧ +3.length ≥ 3<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.974. Support: 134.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 567.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 374.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, if}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 339.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, if}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 26985.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ +3.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 11092.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = .<br>	∧ +2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 12872.` |
| 16 | `  •••start_col ≤ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 5<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {STATEMENT}<br>⇒ y = ⏎⏎<br>Confidence: 0.935. Support: 130.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved not in {}}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ +4.length = 0<br>	∧ ^1.roles not in {STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.984. Support: 96.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {., ;}<br>	∧ +1.reserved = ;<br>	∧ +2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 3982.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = if<br>	∧ +1.reserved = (<br>	∧ +2.roles not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 179.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, if}<br>	∧ +1.reserved = (<br>	∧ +2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 2787.` |
| 21 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {., ;, {}<br>	∧ +1.reserved not in {(, ;}<br>	∧ +2.roles not in {EXPRESSION}<br>⇒ y = ⏎<br>Confidence: 0.980. Support: 1690.` |
| 22 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ +2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1074.` |
| 23 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label in {<newline>}<br>	∧ -1.reserved not in {., ;, {}<br>	∧ +1.reserved not in {(, ;}<br>	∧ +2.roles not in {EXPRESSION}<br>⇒ y = "<br>Confidence: 0.999. Support: 916.` |
| 24 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.reserved = ,<br>	∧ +2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 473.` |
| 25 | `  •••start_col ≤ 23<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {., {}<br>	∧ +1.reserved = }<br>	∧ +2.roles not in {EXPRESSION}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.934. Support: 555.` |
| 26 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {KEY}<br>	∧ +1.reserved not in {,, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 285.` |
| 27 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {KEY}<br>	∧ +2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 1965.` |
| 28 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {KEY}<br>	∧ +1.reserved not in {(, ), ,, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {COMMENT, EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 12686.` |
| 29 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ., ;, {, }}<br>	∧ -1.roles in {IDENTIFIER} and not in {KEY}<br>	∧ +1.reserved not in {(, ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles not in {COMMENT, EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 97.` |
| 30 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {,, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 166.` |
| 31 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -3.reserved = (<br>	∧ -5.diff_col ≤ 6<br>	∧ +1.reserved not in {,, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 190.` |
| 32 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ,, ., ;, {, }}<br>	∧ -1.roles not in {IDENTIFIER, KEY}<br>	∧ -3.reserved not in {(}<br>	∧ -5.label in {<newline>}<br>	∧ +1.reserved not in {(, ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles not in {COMMENT, EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.924. Support: 417.` |
| 33 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ., ;, {, }}<br>	∧ -1.roles not in {IDENTIFIER, KEY}<br>	∧ -3.reserved not in {(}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved not in {(, ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≤ 43<br>	∧ +2.roles not in {COMMENT, EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.948. Support: 15018.` |
| 34 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ., ;, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -3.label not in {"}<br>	∧ -4.label in {<newline>}<br>	∧ +1.reserved not in {(, ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {COMMENT, EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 511.` |
| 35 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ., ;, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -4.diff_offset ≥ 7<br>	∧ -4.label not in {<newline>}<br>	∧ +1.reserved not in {(, ,, ;, }}<br>	∧ +1.roles in {EXPRESSION} and not in {COMMENT}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {COMMENT, EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 1670.` |
| 36 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ., ;, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.label in {<space>}<br>	∧ -4.diff_offset ≤ 6<br>	∧ -4.label not in {<newline>}<br>	∧ +1.reserved not in {(, ,, ;, }}<br>	∧ +1.roles in {BINARY, EXPRESSION} and not in {COMMENT}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {COMMENT, EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 143.` |
| 37 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ., ;, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -4.label not in {<newline>}<br>	∧ +1.reserved not in {(, ,, ;, }}<br>	∧ +1.roles not in {COMMENT, EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {COMMENT, EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 15629.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 10.64864864864865, "max_conf": 0.9999611377716064, "max_support": 26985, "min_conf": 0.9244604110717773, "min_support": 96, "num_rules": 37}}
```
</details>
