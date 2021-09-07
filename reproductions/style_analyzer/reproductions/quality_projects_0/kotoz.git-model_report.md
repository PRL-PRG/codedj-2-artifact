# Model report for file:///tmp/top-repos-quality-repos-j3fdupy1/kotoz.git HEAD 3dd5fd7040864c3f92e014f59ddb1d94245a341c

### Dump

```json
{'created_at': '2021-09-01 18:07:09',
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
 'size': '17.6 kB',
 'tags': [],
 'uuid': '4e9143cc-4856-4923-a63d-1bb0b7c8549a',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-j3fdupy1/kotoz.git 3dd5fd7040864c3f92e014f59ddb1d94245a341c

# javascript
78 rules, avg.len. 6.7
## train
PPCR: 0.961464
### report
macro
{'f1-score': 0.5824529254857125,
 'precision': 0.5852133449228216,
 'recall': 0.5800628626029091,
 'support': 14471}
micro
{'f1-score': 0.9269573630018658,
 'precision': 0.9269573630018658,
 'recall': 0.9269573630018658,
 'support': 14471}
weighted
{'f1-score': 0.9149723590909159,
 'precision': 0.9038173270854551,
 'recall': 0.9269573630018658,
 'support': 14471}
### report_full
macro
{'f1-score': 0.566916723810547,
 'precision': 0.5852133449228216,
 'recall': 0.5530828777572955,
 'support': 15051}
micro
{'f1-score': 0.9087460199173498,
 'precision': 0.9269573630018658,
 'recall': 0.8912364626935088,
 'support': 15051}
weighted
{'f1-score': 0.8880854078819761,
 'precision': 0.8875334276304675,
 'recall': 0.8912364626935088,
 'support': 15051}
## test
PPCR: 0.966970
### report
macro
{'f1-score': 0.5809402031845409,
 'precision': 0.5848246056477361,
 'recall': 0.578268250649523,
 'support': 3191}
micro
{'f1-score': 0.9222814164838609,
 'precision': 0.9222814164838609,
 'recall': 0.9222814164838609,
 'support': 3191}
weighted
{'f1-score': 0.9069025977404891,
 'precision': 0.8926696169259497,
 'recall': 0.9222814164838609,
 'support': 3191}
### report_full
macro
{'f1-score': 0.5707321423268061,
 'precision': 0.5848246056477361,
 'recall': 0.5610127650101826,
 'support': 3300}
micro
{'f1-score': 0.9067940224926823,
 'precision': 0.9222814164838609,
 'recall': 0.8918181818181818,
 'support': 3300}
weighted
{'f1-score': 0.8824323730692716,
 'precision': 0.8753445477439947,
 'recall': 0.8918181818181818,
 'support': 3300}
```

## javascript
### Summary
50 rules, avg.len. 5.7

| | |
|-|-|
|Min support|151|
|Max support|6646|
|Min confidence|0.9259907007217407|
|Max confidence|0.998516321182251|

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
| 1 | `  -1.label in {<space>}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.998. Support: 213.` |
| 2 | `  -1.label not in {<space>}<br>	∧ -3.length ≥ 5<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 647.` |
| 3 | `  -1.label not in {<space>}<br>	∧ -3.label in {<space>}<br>	∧ -3.length ≤ 4<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 201.` |
| 4 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles in {INCOMPLETE}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = "<br>Confidence: 0.997. Support: 174.` |
| 5 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles not in {INCOMPLETE}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.999. Support: 337.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = import<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.972. Support: 161.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.roles in {MAP}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 198.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = import<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 189.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, import}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 4067.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, import}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 1177.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, import}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.926. Support: 858.` |
| 12 | `  +1.internal_type = StringLiteral<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.997. Support: 179.` |
| 13 | `  +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 4653.` |
| 14 | `  ^1.roles in {QUALIFIED} and not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 1150.` |
| 15 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.998. Support: 313.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.998. Support: 297.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = import<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.984. Support: 155.` |
| 18 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 762.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 331.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IF, INCOMPLETE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 795.` |
| 21 | `  -2.reserved = from<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.998. Support: 202.` |
| 22 | `  -2.reserved not in {from}<br>	∧ -3.length ≥ 5<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 631.` |
| 23 | `  -2.reserved not in {from}<br>	∧ -3.label in {<space>}<br>	∧ -3.length ≤ 4<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 235.` |
| 24 | `  -1.internal_type = StringLiteral<br>	∧ -2.reserved = =<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = "<br>Confidence: 0.997. Support: 161.` |
| 25 | `  -1.internal_type = StringLiteral<br>	∧ -2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.998. Support: 323.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {IMPORT} and not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 203.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -3.label not in {<newline>}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = ><br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 252.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -3.label not in {<newline>}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +2.reserved not in {=, >}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 6646.` |
| 29 | `  ^1.roles in {IDENTIFIER} and not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 1171.` |
| 30 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {;}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.998. Support: 304.` |
| 31 | `  -1.internal_type = StringLiteral<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.998. Support: 303.` |
| 32 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.948. Support: 720.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 330.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 809.` |
| 35 | `  ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 1165.` |
| 36 | `  -1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.998. Support: 332.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.998. Support: 306.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = import<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ⏎<br>Confidence: 0.977. Support: 151.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 317.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {CONDITION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 802.` |
| 41 | `  -1.label in {<space>}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.998. Support: 322.` |
| 42 | `  -1.internal_type = StringLiteral<br>	∧ -1.label not in {<space>}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.998. Support: 312.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = import<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ⏎<br>Confidence: 0.971. Support: 153.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;}<br>	∧ -1.length ≥ 3<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.949. Support: 800.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 298.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {PATHNAME}<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 166.` |
| 47 | `  -1.label in {<space>}<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.998. Support: 330.` |
| 48 | `  -1.internal_type = StringLiteral<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;}<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.998. Support: 293.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;}<br>	∧ -1.length ≥ 3<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 768.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;}<br>	∧ -3.length ≥ 5<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {DECLARATION} and not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 422.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.74, "max_conf": 0.998516321182251, "max_support": 6646, "min_conf": 0.9259907007217407, "min_support": 151, "num_rules": 50}}
```
</details>
