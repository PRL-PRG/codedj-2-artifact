# Model report for file:///tmp/top-repos-quality-repos-0iacxk2d/2019fall_42class_team1.git HEAD 0433a527ae869f44b4f342d0075f7a5ab82471b5

### Dump

```json
{'created_at': '2021-09-01 01:29:30',
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
 'size': '20.4 kB',
 'tags': [],
 'uuid': 'a01fa7d3-a92f-4784-9ea7-f37a50d68321',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-0iacxk2d/2019fall_42class_team1.git 0433a527ae869f44b4f342d0075f7a5ab82471b5

# javascript
204 rules, avg.len. 7.3
## train
PPCR: 0.979970
### report
macro
{'f1-score': 0.7718682819182501,
 'precision': 0.800533729894055,
 'recall': 0.7490596354490043,
 'support': 36107}
micro
{'f1-score': 0.9288780568864763,
 'precision': 0.9288780568864763,
 'recall': 0.9288780568864763,
 'support': 36107}
weighted
{'f1-score': 0.9262031546639828,
 'precision': 0.9252443398077668,
 'recall': 0.9288780568864763,
 'support': 36107}
### report_full
macro
{'f1-score': 0.7543101932195142,
 'precision': 0.800533729894055,
 'recall': 0.7190564338305276,
 'support': 36845}
micro
{'f1-score': 0.919481302774427,
 'precision': 0.9288780568864763,
 'recall': 0.9102727642828063,
 'support': 36845}
weighted
{'f1-score': 0.9138551107300472,
 'precision': 0.9211428741847538,
 'recall': 0.9102727642828063,
 'support': 36845}
## test
PPCR: 0.985954
### report
macro
{'f1-score': 0.7631195125685502,
 'precision': 0.7967263898647537,
 'recall': 0.7379477176744965,
 'support': 7581}
micro
{'f1-score': 0.9253396649518533,
 'precision': 0.9253396649518533,
 'recall': 0.9253396649518533,
 'support': 7581}
weighted
{'f1-score': 0.922616446870363,
 'precision': 0.9227923577149835,
 'recall': 0.9253396649518533,
 'support': 7581}
### report_full
macro
{'f1-score': 0.7541805551848568,
 'precision': 0.7967263898647537,
 'recall': 0.7222726218331428,
 'support': 7689}
micro
{'f1-score': 0.9187950229207597,
 'precision': 0.9253396649518533,
 'recall': 0.9123423071920926,
 'support': 7689}
weighted
{'f1-score': 0.9134035135565477,
 'precision': 0.9182600536961745,
 'recall': 0.9123423071920926,
 'support': 7689}
```

## javascript
### Summary
99 rules, avg.len. 7.1

| | |
|-|-|
|Min support|139|
|Max support|13569|
|Min confidence|0.920621931552887|
|Max confidence|0.9991196990013123|

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
| 1 | `  •••start_line ≥ 76<br>	∧ -1.internal_type = StringLiteral<br>	∧ -2.reserved = =<br>⇒ y = "<br>Confidence: 0.976. Support: 312.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ +1.roles in {LITERAL}<br>⇒ y = "<br>Confidence: 0.960. Support: 316.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.978. Support: 4989.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.990. Support: 3772.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 592.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = ObjectExpression<br>⇒ y = ⏎<br>Confidence: 0.930. Support: 604.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -5.reserved = from<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression, ObjectExpression}<br>⇒ y = ⏎<br>Confidence: 0.933. Support: 408.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles in {OPERATOR} and not in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 170.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 3713.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = =<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 650.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 481.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.921. Support: 500.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, =}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 182.` |
| 14 | `  •••start_line ≥ 53<br>	∧ -1.internal_type = StringLiteral<br>	∧ -2.reserved = =<br>	∧ -4.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.975. Support: 337.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.roles in {KEY, LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.978. Support: 157.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.length ≤ 4<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^2.roles in {BODY}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.984. Support: 278.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.length ≤ 4<br>	∧ -4.diff_offset ≤ 4<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.954. Support: 248.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -5.roles in {FUNCTION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 284.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -5.roles not in {FUNCTION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 231.` |
| 20 | `  -1.internal_type = StringLiteral<br>	∧ -2.label in {<space>}<br>	∧ -4.roles not in {STRING}<br>⇒ y = '<br>Confidence: 0.921. Support: 808.` |
| 21 | `  -1.internal_type = StringLiteral<br>	∧ -2.label not in {<space>}<br>	∧ -2.reserved = =<br>⇒ y = "<br>Confidence: 0.967. Support: 348.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 4898.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 3934.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.diff_col ≤ 6<br>	∧ -4.diff_offset ≥ 5<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {BODY}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.988. Support: 215.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.diff_col ≤ 6<br>	∧ -4.diff_offset ≤ 4<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.986. Support: 333.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 568.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.921. Support: 649.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.928. Support: 604.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 3843.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -5.roles in {FUNCTION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 280.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -5.roles not in {FUNCTION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 229.` |
| 32 | `  -1.label not in {<space>}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ +1.roles in {LITERAL}<br>⇒ y = "<br>Confidence: 0.966. Support: 369.` |
| 33 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.roles in {KEY, LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.975. Support: 139.` |
| 34 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles in {INCOMPLETE}<br>	∧ +1.roles not in {LITERAL}<br>⇒ y = "<br>Confidence: 0.974. Support: 328.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 4836.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 3828.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.length ≤ 4<br>	∧ -4.diff_offset ≥ 5<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles in {BODY}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.969. Support: 207.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.length ≤ 4<br>	∧ -4.diff_offset ≤ 4<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.985. Support: 306.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 561.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = import<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, ObjectExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.928. Support: 353.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED, VARIABLE}<br>	∧ ^2.internal_type = JSXOpeningElement<br>⇒ y = ∅<br>Confidence: 0.999. Support: 429.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED, VARIABLE}<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 203.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 5454.` |
| 44 | `  -1.label in {<space>}<br>	∧ -3.roles not in {STRING}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = '<br>Confidence: 0.924. Support: 796.` |
| 45 | `  -1.label not in {<space>}<br>	∧ -2.roles in {INCOMPLETE}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = "<br>Confidence: 0.973. Support: 353.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 173.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 149.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {BLOCK} and not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 756.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 584.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.reserved = import<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles in {FILE}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 355.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -4.roles in {PATHNAME}<br>	∧ +1.reserved not in {import}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles in {FILE}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 325.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -4.roles not in {PATHNAME}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles in {FILE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.953. Support: 201.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -4.roles not in {PATHNAME}<br>	∧ +1.reserved not in {import, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 2<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles in {FILE}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 279.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles in {BODY} and not in {FILE}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.960. Support: 290.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles in {BODY, EXPRESSION} and not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.921. Support: 209.` |
| 56 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {BODY, FILE}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 469.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {BLOCK, DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {BODY, FILE}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 13569.` |
| 58 | `  -1.internal_type = StringLiteral<br>	∧ -2.reserved = =<br>⇒ y = "<br>Confidence: 0.977. Support: 364.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.diff_col ≤ 6<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {BODY}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.983. Support: 325.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.diff_col ≤ 6<br>	∧ -4.diff_offset ≤ 4<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.984. Support: 219.` |
| 61 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -1.length ≤ 5<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ -5.roles not in {FUNCTION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 927.` |
| 62 | `  •••start_line ≤ 93<br>	∧ -1.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.934. Support: 808.` |
| 63 | `  -1.diff_offset ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles in {EXPRESSION} and not in {INCOMPLETE}<br>⇒ y = ⏎<br>Confidence: 0.978. Support: 156.` |
| 64 | `  -1.internal_type = StringLiteral<br>	∧ -2.reserved = =<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = "<br>Confidence: 0.963. Support: 335.` |
| 65 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 4870.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 3870.` |
| 67 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.length ≤ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {BODY}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.981. Support: 283.` |
| 68 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.length ≤ 4<br>	∧ -4.diff_offset ≤ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.970. Support: 214.` |
| 69 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 610.` |
| 70 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 3708.` |
| 71 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = =<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 695.` |
| 72 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 555.` |
| 73 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 505.` |
| 74 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, =}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 187.` |
| 75 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, =, }}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {,, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.921. Support: 611.` |
| 76 | `  -1.label in {<space>}<br>	∧ -3.internal_type not in {StringLiteral}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.937. Support: 786.` |
| 77 | `  -1.label not in {<space>}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.969. Support: 340.` |
| 78 | `  -1.label not in {<space>}<br>	∧ -1.length ≤ 4<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles in {EXPRESSION}<br>⇒ y = ⏎<br>Confidence: 0.983. Support: 148.` |
| 79 | `  -1.internal_type = StringLiteral<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.internal_type = Identifier<br>⇒ y = '<br>Confidence: 0.957. Support: 590.` |
| 80 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles in {INCOMPLETE}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.internal_type not in {Identifier}<br>	∧ +3.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.961. Support: 342.` |
| 81 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.diff_col ≤ 6<br>	∧ -4.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {BODY}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.971. Support: 223.` |
| 82 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.diff_col ≤ 6<br>	∧ -4.diff_offset ≤ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.983. Support: 319.` |
| 83 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -5.reserved = from<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, ObjectExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.921. Support: 386.` |
| 84 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {INCOMPLETE} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 143.` |
| 85 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -5.roles in {FUNCTION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 305.` |
| 86 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -5.roles not in {FUNCTION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 244.` |
| 87 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {INCOMPLETE}<br>	∧ +1.roles in {KEY, LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.969. Support: 146.` |
| 88 | `  •••start_line ≥ 76<br>	∧ -1.internal_type = StringLiteral<br>	∧ -3.internal_type = JSXIdentifier<br>	∧ +1.roles not in {LITERAL}<br>⇒ y = "<br>Confidence: 0.971. Support: 291.` |
| 89 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.length ≤ 4<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {BODY}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.970. Support: 320.` |
| 90 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.length ≤ 4<br>	∧ -4.diff_offset ≤ 4<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.988. Support: 200.` |
| 91 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), :}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -1.length ≤ 1<br>	∧ -2.length ≤ 1<br>	∧ -5.roles not in {FUNCTION}<br>	∧ +1.reserved = ><br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 249.` |
| 92 | `  -1.internal_type = StringLiteral<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.internal_type = Identifier<br>⇒ y = '<br>Confidence: 0.951. Support: 600.` |
| 93 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type = JSXIdentifier<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.internal_type not in {Identifier}<br>	∧ +3.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.971. Support: 327.` |
| 94 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.length ≤ 4<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles in {BODY}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.991. Support: 289.` |
| 95 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.length ≤ 4<br>	∧ -4.diff_offset ≤ 4<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.967. Support: 226.` |
| 96 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.923. Support: 580.` |
| 97 | `  -1.internal_type not in {StringLiteral}<br>	∧ -5.roles in {FUNCTION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 320.` |
| 98 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -5.roles not in {FUNCTION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 233.` |
| 99 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ -3.reserved not in {'}<br>	∧ -5.roles not in {FUNCTION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 3712.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.08080808080808, "max_conf": 0.9991196990013123, "max_support": 13569, "min_conf": 0.920621931552887, "min_support": 139, "num_rules": 99}}
```
</details>
