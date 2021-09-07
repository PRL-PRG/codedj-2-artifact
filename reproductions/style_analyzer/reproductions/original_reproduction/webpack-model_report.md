# Model report for file:///tmp/top-repos-quality-repos-7hpa0wxp/webpack HEAD babe736cfa1ef7e8014ed32ba4a4ec38049dce14

### Dump

```json
{'created_at': '2021-08-14 10:54:44',
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
 'size': '23.4 kB',
 'tags': [],
 'uuid': 'fd6d9eaf-e3c4-4d93-b759-7e61ac945436',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-7hpa0wxp/webpack babe736cfa1ef7e8014ed32ba4a4ec38049dce14

# javascript
109 rules, avg.len. 10.3
## train
PPCR: 0.949290
### report
macro
{'f1-score': 0.5750142644959401,
 'precision': 0.6087973232740608,
 'recall': 0.5500374882143371,
 'support': 294259}
micro
{'f1-score': 0.9451707509370997,
 'precision': 0.9451707509370997,
 'recall': 0.9451707509370997,
 'support': 294259}
weighted
{'f1-score': 0.941214742423377,
 'precision': 0.939348955147577,
 'recall': 0.9451707509370997,
 'support': 294259}
### report_full
macro
{'f1-score': 0.5328019283661469,
 'precision': 0.6087973232740608,
 'recall': 0.48678463724561677,
 'support': 309978}
micro
{'f1-score': 0.9205824866732756,
 'precision': 0.9451707509370997,
 'recall': 0.8972410945292891,
 'support': 309978}
weighted
{'f1-score': 0.911773205855583,
 'precision': 0.9342884942953551,
 'recall': 0.8972410945292891,
 'support': 309978}
## test
PPCR: 0.946194
### report
macro
{'f1-score': 0.5238570235462552,
 'precision': 0.544431965144844,
 'recall': 0.5065344372836948,
 'support': 73084}
micro
{'f1-score': 0.9474440369985222,
 'precision': 0.9474440369985222,
 'recall': 0.9474440369985222,
 'support': 73084}
weighted
{'f1-score': 0.9453106092168684,
 'precision': 0.9451588744542627,
 'recall': 0.9474440369985222,
 'support': 73084}
### report_full
macro
{'f1-score': 0.485911075500153,
 'precision': 0.544431965144844,
 'recall': 0.4460242465327066,
 'support': 77240}
micro
{'f1-score': 0.9212500997844655,
 'precision': 0.9474440369985222,
 'recall': 0.8964655618850337,
 'support': 77240}
weighted
{'f1-score': 0.9159195724024126,
 'precision': 0.944174376001921,
 'recall': 0.8964655618850337,
 'support': 77240}
```

## javascript
### Summary
109 rules, avg.len. 10.3

| | |
|-|-|
|Min support|90|
|Max support|35271|
|Min confidence|0.8175965547561646|
|Max confidence|0.9993908405303955|

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
| 1 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.942. Support: 718.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ +4.length ≥ 9<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.851. Support: 138.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ +4.length ≤ 8<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.886. Support: 541.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -3.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.907. Support: 2558.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {)}<br>	∧ -3.internal_type not in {StringLiteral}<br>	∧ -4.reserved = .<br>	∧ +1.reserved not in {}}<br>	∧ +2.length ≥ 4<br>	∧ +4.roles in {ARGUMENT}<br>	∧ +5.reserved = )<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 113.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {)}<br>	∧ -3.internal_type not in {StringLiteral}<br>	∧ -4.reserved = .<br>	∧ +1.reserved not in {}}<br>	∧ +4.roles in {ARGUMENT}<br>	∧ +5.reserved not in {)}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 346.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {)}<br>	∧ -3.internal_type not in {StringLiteral}<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {}}<br>	∧ +4.roles in {ARGUMENT}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 3281.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {)}<br>	∧ -3.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +4.roles not in {ARGUMENT}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 35271.` |
| 9 | `  •••start_line = 255<br>	∧ -1.internal_type = StringLiteral<br>	∧ -2.reserved = (<br>	∧ -5.diff_offset ≥ 22<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.internal_type = ExpressionStatement<br>⇒ y = "<br>Confidence: 0.865. Support: 130.` |
| 10 | `  •••start_line = 255<br>	∧ -1.internal_type = StringLiteral<br>	∧ -2.reserved not in {(}<br>	∧ -5.diff_offset ≥ 22<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.943. Support: 694.` |
| 11 | `  •••start_line = 255<br>	∧ -1.internal_type = StringLiteral<br>	∧ -2.reserved not in {(}<br>	∧ -5.diff_offset ≤ 21<br>	∧ ^1.roles in {LITERAL} and not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.934. Support: 98.` |
| 12 | `  •••start_line = 255<br>	∧ -1.internal_type = StringLiteral<br>	∧ -2.reserved not in {(}<br>	∧ -5.diff_offset ≤ 21<br>	∧ ^1.roles in {IF} and not in {LITERAL, QUALIFIED}<br>⇒ y = "<br>Confidence: 0.823. Support: 99.` |
| 13 | `  •••start_line ≤ 254<br>	∧ -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.975. Support: 9244.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.reserved = )<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.833. Support: 224.` |
| 15 | `  •••start_line ≤ 46<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.reserved not in {)}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {,, }}<br>	∧ ^1.internal_type = FunctionDeclaration<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.864. Support: 143.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.reserved not in {)}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {,, }}<br>	∧ ^1.internal_type not in {FunctionDeclaration}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.956. Support: 7572.` |
| 17 | `  •••start_line ≤ 254<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≤ 13<br>	∧ +3.length ≥ 9<br>	∧ +4.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.821. Support: 126.` |
| 18 | `  •••start_line ≤ 254<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ +3.length ≤ 8<br>	∧ +4.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.960. Support: 4032.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -4.diff_offset ≥ 24<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = )<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 346.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -4.diff_offset ≤ 23<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 9819.` |
| 21 | `  •••start_col ≥ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.876. Support: 5346.` |
| 22 | `  •••start_col ≤ 10<br>	∧ •••start_line ≤ 126<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ +3.roles in {STRING}<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.830. Support: 150.` |
| 23 | `  •••start_col ≥ 15<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 3<br>	∧ +2.roles in {NAME}<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.984. Support: 91.` |
| 24 | `  •••start_col ≤ 14<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -3.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.897. Support: 92.` |
| 25 | `  •••start_col ≤ 14<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -3.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.824. Support: 964.` |
| 26 | `  •••start_line = 255<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -4.diff_offset ≥ 9<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.843. Support: 538.` |
| 27 | `  •••start_line ≤ 254<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.963. Support: 3433.` |
| 28 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL, STRING} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 1611.` |
| 29 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ +1.roles in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {QUALIFIED, STRING}<br>⇒ y = ⏎<br>Confidence: 0.946. Support: 2058.` |
| 30 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -5.label in {<newline>}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {QUALIFIED, STRING}<br>⇒ y = ⏎<br>Confidence: 0.973. Support: 457.` |
| 31 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -3.roles in {LITERAL}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {QUALIFIED, STRING}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 270.` |
| 32 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ -3.roles not in {LITERAL}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {QUALIFIED, STRING}<br>⇒ y = ⏎<br>Confidence: 0.898. Support: 162.` |
| 33 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -3.roles in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.929. Support: 161.` |
| 34 | `  •••start_col ≥ 6<br>	∧ -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.919. Support: 824.` |
| 35 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -4.label not in {<newline>}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.858. Support: 530.` |
| 36 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.length ≤ 14<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 1843.` |
| 37 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = [<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≤ 14<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = "<br>Confidence: 0.832. Support: 419.` |
| 38 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = "<br>Confidence: 0.979. Support: 362.` |
| 39 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {STATEMENT}<br>	∧ -2.length ≥ 3<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.827. Support: 113.` |
| 40 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {STATEMENT}<br>	∧ -2.length ≤ 2<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.931. Support: 124.` |
| 41 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = }<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {BLOCK, LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.941. Support: 297.` |
| 42 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = ,<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.label in {<newline>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.954. Support: 571.` |
| 43 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = ,<br>	∧ -2.roles in {ARGUMENT}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.label not in {<newline>}<br>	∧ -5.label in {<newline>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 271.` |
| 44 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.label not in {<newline>}<br>	∧ -5.label in {<newline>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.946. Support: 267.` |
| 45 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = ,<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.label not in {<newline>}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 3319.` |
| 46 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.roles in {VALUE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {CALL} and not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 124.` |
| 47 | `  •••start_col ≥ 6<br>	∧ -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {CALL} and not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 848.` |
| 48 | `  •••start_col ≥ 24<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {CALL} and not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.861. Support: 184.` |
| 49 | `  •••start_col ≤ 62<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -3.diff_offset ≥ 9<br>	∧ -4.label in {<newline>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 1363.` |
| 50 | `  •••start_col ≤ 62<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -3.diff_offset ≤ 8<br>	∧ -3.length ≤ 5<br>	∧ -4.label in {<newline>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 447.` |
| 51 | `  •••start_col ≤ 62<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -4.label not in {<newline>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 17818.` |
| 52 | `  •••start_col ≥ 6<br>	∧ -1.diff_offset ≥ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -5.label not in {<newline>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 213.` |
| 53 | `  •••start_col ≥ 6<br>	∧ -1.diff_offset ≤ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, [, {}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.reserved not in {;}<br>	∧ -3.length ≥ 2<br>	∧ -5.label not in {<newline>}<br>	∧ +1.roles not in {VALUE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = LogicalExpression<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.883. Support: 90.` |
| 54 | `  •••start_col ≥ 6<br>	∧ -1.diff_offset ≤ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.roles in {EXPRESSION}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.reserved not in {;}<br>	∧ -4.diff_col ≤ 25<br>	∧ -5.label in {<space>} and not in {<newline>}<br>	∧ +1.roles not in {VALUE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.861. Support: 183.` |
| 55 | `  •••start_col ≥ 6<br>	∧ -1.diff_offset ≤ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.reserved not in {;}<br>	∧ -4.diff_col ≤ 25<br>	∧ -5.label not in {<newline>, <space>}<br>	∧ +1.roles not in {VALUE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.884. Support: 2504.` |
| 56 | `  •••start_col ≤ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.992. Support: 910.` |
| 57 | `  •••start_col ≤ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = }<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {STATEMENT} and not in {QUALIFIED, SCOPE}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 306.` |
| 58 | `  •••start_col ≤ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = }<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED, STATEMENT}<br>⇒ y = ⏎⏎<br>Confidence: 0.886. Support: 881.` |
| 59 | `  •••start_col ≤ 5<br>	∧ •••start_line ≥ 211<br>	∧ -1.diff_col ≥ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.818. Support: 233.` |
| 60 | `  •••start_col ≤ 5<br>	∧ •••start_line ≤ 211<br>	∧ -1.diff_col ≥ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.905. Support: 384.` |
| 61 | `  •••start_col ≤ 5<br>	∧ -1.diff_col ≤ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 558.` |
| 62 | `  •••start_col ≤ 5<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = ,<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {MAP} and not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.942. Support: 318.` |
| 63 | `  •••start_col ≤ 5<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = ,<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {MAP, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.824. Support: 94.` |
| 64 | `  •••start_col ≤ 5<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 3396.` |
| 65 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 9029.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = TemplateLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 586.` |
| 67 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {DECLARATION} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.904. Support: 329.` |
| 68 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {IDENTIFIER, MAP}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {TemplateLiteral}<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.826. Support: 141.` |
| 69 | `  •••start_line ≤ 240<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +4.internal_type not in {CommentBlock}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.954. Support: 274.` |
| 70 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +4.internal_type not in {CommentBlock}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.926. Support: 7060.` |
| 71 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.901. Support: 460.` |
| 72 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = ArrayExpression<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.890. Support: 150.` |
| 73 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = [<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.939. Support: 106.` |
| 74 | `  •••start_line ≤ 235<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, [}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = FunctionExpression<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 442.` |
| 75 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, [}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 7213.` |
| 76 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {EXPRESSION, STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.927. Support: 296.` |
| 77 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 1086.` |
| 78 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.890. Support: 659.` |
| 79 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {STRING} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 220.` |
| 80 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.label in {<newline>}<br>	∧ -5.reserved = )<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 92.` |
| 81 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -5.reserved not in {)}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.880. Support: 3254.` |
| 82 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 2215.` |
| 83 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = BinaryExpression<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 1462.` |
| 84 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = BinaryExpression<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.826. Support: 382.` |
| 85 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -5.reserved = (<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 444.` |
| 86 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -3.length ≥ 2<br>	∧ -5.reserved not in {(}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.891. Support: 179.` |
| 87 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 466.` |
| 88 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if}<br>	∧ -2.internal_type = StringLiteral<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {CONDITION, QUALIFIED}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.888. Support: 282.` |
| 89 | `  -1.internal_type = DirectiveLiteral<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.998. Support: 252.` |
| 90 | `  -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 243.` |
| 91 | `  -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles in {CONDITION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 202.` |
| 92 | `  -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved = =<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {VARIABLE} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.901. Support: 248.` |
| 93 | `  •••start_line ≤ 212<br>	∧ -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :, =}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles in {VARIABLE} and not in {CONDITION}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 111.` |
| 94 | `  -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.length ≥ 88<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {CONDITION, QUALIFIED, VARIABLE}<br>⇒ y = ⏎<br>Confidence: 0.902. Support: 128.` |
| 95 | `  -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles in {OPERATOR} and not in {CONDITION, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 380.` |
| 96 | `  -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.diff_offset ≤ 3<br>	∧ -3.diff_offset ≥ 7<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.893. Support: 211.` |
| 97 | `  •••start_col ≥ 13<br>	∧ -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {CONDITION, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.892. Support: 707.` |
| 98 | `  •••start_col ≤ 12<br>	∧ -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {CONDITION, OPERATOR, QUALIFIED, VARIABLE}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.962. Support: 197.` |
| 99 | `  -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.length ≥ 4<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = ;<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {CONDITION, OPERATOR, QUALIFIED, VARIABLE}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.991. Support: 176.` |
| 100 | `  -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.length ≥ 4<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = ;<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {CONDITION, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 687.` |
| 101 | `  -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.length ≤ 3<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = ;<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {CONDITION, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 5796.` |
| 102 | `  •••start_col ≥ 44<br>	∧ -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {;}<br>	∧ ^1.internal_type = TemplateLiteral<br>	∧ ^1.roles not in {CONDITION, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.931. Support: 94.` |
| 103 | `  -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -3.length ≥ 5<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {;}<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {CONDITION, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 361.` |
| 104 | `  -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -5.diff_line = 0<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {;}<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {CONDITION, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 3926.` |
| 105 | `  -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {;}<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {CONDITION, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 28768.` |
| 106 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 3<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {;}<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {CONDITION, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 1190.` |
| 107 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 3<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {;}<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.878. Support: 602.` |
| 108 | `  -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {;}<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {CONDITION, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 20444.` |
| 109 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length = 0<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.866. Support: 1878.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 10.284403669724771, "max_conf": 0.9993908405303955, "max_support": 35271, "min_conf": 0.8175965547561646, "min_support": 90, "num_rules": 109}}
```
</details>
