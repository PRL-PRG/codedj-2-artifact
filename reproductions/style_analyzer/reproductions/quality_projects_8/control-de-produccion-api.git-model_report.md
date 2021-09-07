# Model report for file:///tmp/top-repos-quality-repos-520y5oqb/control-de-produccion-api.git HEAD 0988c2224a2dc61e5c7e83a45480a7c516772db0

### Dump

```json
{'created_at': '2021-09-01 00:01:17',
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
 'size': '23.4 kB',
 'tags': [],
 'uuid': '90ad6ad1-a8a7-402a-8849-b3f71189019d',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-520y5oqb/control-de-produccion-api.git 0988c2224a2dc61e5c7e83a45480a7c516772db0

# javascript
219 rules, avg.len. 8.3
## train
PPCR: 0.941052
### report
macro
{'f1-score': 0.6311135498907461,
 'precision': 0.7091185809059382,
 'recall': 0.5916126210507123,
 'support': 57311}
micro
{'f1-score': 0.9182530404285391,
 'precision': 0.9182530404285391,
 'recall': 0.9182530404285391,
 'support': 57311}
weighted
{'f1-score': 0.9083876462583742,
 'precision': 0.9062411480951593,
 'recall': 0.9182530404285391,
 'support': 57311}
### report_full
macro
{'f1-score': 0.5616453075221575,
 'precision': 0.7091185809059382,
 'recall': 0.5044816453709708,
 'support': 60901}
micro
{'f1-score': 0.8903664602578419,
 'precision': 0.9182530404285391,
 'recall': 0.8641237418104794,
 'support': 60901}
weighted
{'f1-score': 0.8722891950836068,
 'precision': 0.9036968087134862,
 'recall': 0.8641237418104794,
 'support': 60901}
## test
PPCR: 0.948892
### report
macro
{'f1-score': 0.6028256071426004,
 'precision': 0.6843957309430595,
 'recall': 0.5611704145103573,
 'support': 13702}
micro
{'f1-score': 0.8826448693621369,
 'precision': 0.8826448693621369,
 'recall': 0.8826448693621369,
 'support': 13702}
weighted
{'f1-score': 0.8704588076160947,
 'precision': 0.8703551985653236,
 'recall': 0.8826448693621369,
 'support': 13702}
### report_full
macro
{'f1-score': 0.5397645702380623,
 'precision': 0.6843957309430595,
 'recall': 0.4828240411227063,
 'support': 14440}
micro
{'f1-score': 0.8594982588302182,
 'precision': 0.8826448693621369,
 'recall': 0.8375346260387811,
 'support': 14440}
weighted
{'f1-score': 0.8402594806078505,
 'precision': 0.8672161157818037,
 'recall': 0.8375346260387811,
 'support': 14440}
```

## javascript
### Summary
106 rules, avg.len. 7.0

| | |
|-|-|
|Min support|135|
|Max support|11401|
|Min confidence|0.9203625321388245|
|Max confidence|0.9997113347053528|

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
                     'min_samples_split': 182,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.reserved not in {)}<br>	∧ +1.reserved not in {], }}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 8514.` |
| 2 | `  -1.internal_type = Identifier<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1684.` |
| 3 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {EXPRESSION} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 158.` |
| 4 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 7594.` |
| 5 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 2583.` |
| 6 | `  -1.internal_type not in {CommentLine, Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, )}<br>	∧ -3.diff_col ≥ 4<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.923. Support: 5526.` |
| 7 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label in {<newline>}<br>	∧ -1.reserved not in {(}<br>	∧ -3.diff_col ≤ 3<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.993. Support: 226.` |
| 8 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = {<br>	∧ -3.diff_col ≤ 3<br>	∧ -5.diff_col ≤ 14<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.940. Support: 159.` |
| 9 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.reserved = (<br>	∧ -3.diff_col ≤ 3<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 191.` |
| 10 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(, [}<br>	∧ -2.reserved not in {}}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 1398.` |
| 11 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 921.` |
| 12 | `  -1.reserved = )<br>	∧ +1.reserved = }<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.991. Support: 169.` |
| 13 | `  -1.reserved not in {)}<br>	∧ +1.reserved not in {], }}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 8786.` |
| 14 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1669.` |
| 15 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 153.` |
| 16 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 7568.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 2487.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.923. Support: 1207.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, )}<br>	∧ -1.roles not in {COMMENT, IDENTIFIER}<br>	∧ -3.diff_col ≥ 4<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.920. Support: 5682.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -3.diff_col ≤ 3<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.993. Support: 223.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -3.diff_col ≤ 3<br>	∧ -5.diff_col ≤ 17<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles in {ARGUMENT} and not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.922. Support: 212.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.reserved = (<br>	∧ -3.diff_col ≤ 3<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 155.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, [}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.reserved not in {}}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 1381.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.reserved = )<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.930. Support: 135.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 949.` |
| 26 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {BINARY} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 182.` |
| 27 | `  •••start_line ≥ 68<br>	∧ -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.989. Support: 1352.` |
| 28 | `  •••start_line ≤ 67<br>	∧ -1.internal_type = StringLiteral<br>	∧ +2.length ≥ 3<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.924. Support: 469.` |
| 29 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 2467.` |
| 30 | `  •••start_line ≥ 21<br>	∧ -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.964. Support: 1081.` |
| 31 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label in {<newline>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.982. Support: 254.` |
| 32 | `  -1.internal_type not in {CommentLine, Identifier, StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ), {}<br>	∧ -3.diff_offset ≤ 12<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 448.` |
| 33 | `  -1.internal_type not in {CommentLine, Identifier, StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ), {}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {CallExpression, ObjectExpression}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 4983.` |
| 34 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.label not in {<space>}<br>	∧ -2.reserved not in {)}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.940. Support: 376.` |
| 35 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {EXPRESSION} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 159.` |
| 36 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 7634.` |
| 37 | `  •••start_line ≥ 39<br>	∧ -1.internal_type = StringLiteral<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.977. Support: 1498.` |
| 38 | `  •••start_line ≤ 38<br>	∧ -1.internal_type = StringLiteral<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +2.length ≥ 3<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.934. Support: 325.` |
| 39 | `  •••start_line ≥ 20<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.959. Support: 1150.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.994. Support: 263.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ), {}<br>	∧ -1.roles not in {COMMENT, IDENTIFIER}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {CallExpression, ObjectExpression}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 5029.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, [}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.reserved not in {}}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 1431.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.967. Support: 375.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 961.` |
| 45 | `  -1.reserved = )<br>	∧ +1.reserved = }<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.944. Support: 187.` |
| 46 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 7716.` |
| 47 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, )}<br>	∧ -1.roles not in {COMMENT}<br>	∧ -3.diff_col ≥ 4<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 5653.` |
| 48 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -3.diff_col ≤ 3<br>	∧ -5.diff_col ≤ 14<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type = CallExpression<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.931. Support: 252.` |
| 49 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label in {<newline>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.diff_col ≤ 3<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.998. Support: 220.` |
| 50 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.961. Support: 371.` |
| 51 | `  -1.reserved not in {)}<br>	∧ +1.reserved not in {], }}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.986. Support: 8641.` |
| 52 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1710.` |
| 53 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {BINARY}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 169.` |
| 54 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 7586.` |
| 55 | `  •••start_line ≥ 36<br>	∧ -1.internal_type = StringLiteral<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = "<br>Confidence: 0.976. Support: 1532.` |
| 56 | `  •••start_line ≤ 35<br>	∧ -1.internal_type = StringLiteral<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +5.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = "<br>Confidence: 0.962. Support: 279.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 2567.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -3.length ≤ 2<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 205.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.948. Support: 2496.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, :}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1343.` |
| 61 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, :}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {,, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 940.` |
| 62 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, :}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ +1.roles not in {COMMENT, EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 722.` |
| 63 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, [}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.reserved not in {}}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >, }}<br>	∧ +1.roles not in {COMMENT, EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 957.` |
| 64 | `  •••start_line ≥ 21<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, [}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {COMMENT, EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 177.` |
| 65 | `  -1.internal_type = Identifier<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1557.` |
| 66 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {BINARY}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 177.` |
| 67 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 11401.` |
| 68 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = .<br>⇒ y = ∅<br>Confidence: 0.999. Support: 3853.` |
| 69 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = :<br>⇒ y = ␣<br>Confidence: 0.992. Support: 2572.` |
| 70 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -3.length ≤ 2<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.979. Support: 214.` |
| 71 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 2794.` |
| 72 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -5.diff_col ≤ 12<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^2.internal_type = CallExpression<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.933. Support: 245.` |
| 73 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = [<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 253.` |
| 74 | `  •••start_col ≤ 19<br>	∧ -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(, ., [, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved not in {(}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 2174.` |
| 75 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -3.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved = ><br>⇒ y = ␣<br>Confidence: 0.998. Support: 255.` |
| 76 | `  -1.internal_type = Identifier<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1662.` |
| 77 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 168.` |
| 78 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 7351.` |
| 79 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = :<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 2659.` |
| 80 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -3.length ≤ 2<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 210.` |
| 81 | `  •••start_line ≤ 52<br>	∧ -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +5.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = "<br>Confidence: 0.977. Support: 241.` |
| 82 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 2533.` |
| 83 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(, :}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +4.reserved = ,<br>	∧ +5.reserved not in {,}<br>	∧ ^1.internal_type = ObjectExpression<br>⇒ y = ⏎<br>Confidence: 0.949. Support: 539.` |
| 84 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(, ), :, {}<br>	∧ -2.diff_offset ≤ 16<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.label not in {<newline>}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression, ObjectExpression}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 3047.` |
| 85 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(, :}<br>	∧ -2.reserved = )<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.943. Support: 149.` |
| 86 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(, :}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles not in {MAP}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 750.` |
| 87 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(, :}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 962.` |
| 88 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 267.` |
| 89 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -5.diff_col ≤ 52<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1234.` |
| 90 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -5.diff_col ≤ 52<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 727.` |
| 91 | `  -1.internal_type = Identifier<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1732.` |
| 92 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 164.` |
| 93 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 7541.` |
| 94 | `  •••start_line ≥ 21<br>	∧ -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.966. Support: 1626.` |
| 95 | `  •••start_line ≤ 20<br>	∧ -1.internal_type = StringLiteral<br>	∧ +2.length ≥ 3<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.931. Support: 270.` |
| 96 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 2591.` |
| 97 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.921. Support: 1239.` |
| 98 | `  -1.internal_type not in {CommentLine, Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, )}<br>	∧ -3.diff_col ≥ 4<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 5681.` |
| 99 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.diff_line ≥ 1<br>	∧ -3.diff_col ≤ 3<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.989. Support: 225.` |
| 100 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.diff_line = 0<br>	∧ -2.reserved = (<br>	∧ -3.diff_col ≤ 3<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 164.` |
| 101 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(, [}<br>	∧ -2.reserved not in {}}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 1386.` |
| 102 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -2.reserved = )<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.926. Support: 155.` |
| 103 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 922.` |
| 104 | `  •••start_line ≥ 65<br>	∧ -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.982. Support: 1327.` |
| 105 | `  •••start_line ≤ 64<br>	∧ -1.internal_type = StringLiteral<br>	∧ +2.length ≥ 3<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.924. Support: 453.` |
| 106 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -3.diff_col ≤ 3<br>	∧ -5.diff_col ≤ 15<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type = CallExpression<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.940. Support: 226.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.018867924528302, "max_conf": 0.9997113347053528, "max_support": 11401, "min_conf": 0.9203625321388245, "min_support": 135, "num_rules": 106}}
```
</details>
