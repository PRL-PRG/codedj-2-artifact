# Model report for file:///tmp/top-repos-quality-repos-j193rcti/avalon.git HEAD f8e2d458993647a98853a84222b480012ae16e39

### Dump

```json
{'created_at': '2021-08-31 20:12:35',
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
 'size': '17.8 kB',
 'tags': [],
 'uuid': '1fa9ac54-1b41-48f2-9846-510fb7a87014',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-j193rcti/avalon.git f8e2d458993647a98853a84222b480012ae16e39

# javascript
104 rules, avg.len. 7.2
## train
PPCR: 0.949496
### report
macro
{'f1-score': 0.6648014226991422,
 'precision': 0.7710076292301127,
 'recall': 0.6088492753469207,
 'support': 13010}
micro
{'f1-score': 0.8980784012298232,
 'precision': 0.8980784012298232,
 'recall': 0.8980784012298232,
 'support': 13010}
weighted
{'f1-score': 0.8893089372472665,
 'precision': 0.8986524807619221,
 'recall': 0.8980784012298232,
 'support': 13010}
### report_full
macro
{'f1-score': 0.6477313639630202,
 'precision': 0.7710076292301127,
 'recall': 0.5846109299351655,
 'support': 13702}
micro
{'f1-score': 0.8748128182090446,
 'precision': 0.8980784012298232,
 'recall': 0.8527222303313385,
 'support': 13702}
weighted
{'f1-score': 0.8522620199113881,
 'precision': 0.8732956265837022,
 'recall': 0.8527222303313385,
 'support': 13702}
## test
PPCR: 0.944623
### report
macro
{'f1-score': 0.6417013563162911,
 'precision': 0.7496293141344751,
 'recall': 0.5864801976612107,
 'support': 2644}
micro
{'f1-score': 0.8944780635400907,
 'precision': 0.8944780635400907,
 'recall': 0.8944780635400907,
 'support': 2644}
weighted
{'f1-score': 0.8848918682609901,
 'precision': 0.8983920053078155,
 'recall': 0.8944780635400907,
 'support': 2644}
### report_full
macro
{'f1-score': 0.626694187450246,
 'precision': 0.7496293141344751,
 'recall': 0.5665360878816047,
 'support': 2799}
micro
{'f1-score': 0.8690060628329964,
 'precision': 0.8944780635400907,
 'recall': 0.8449446230796713,
 'support': 2799}
weighted
{'f1-score': 0.8413379170712548,
 'precision': 0.8653146989486187,
 'recall': 0.8449446230796713,
 'support': 2799}
```

## javascript
### Summary
61 rules, avg.len. 6.7

| | |
|-|-|
|Min support|127|
|Max support|2127|
|Min confidence|0.9203125238418579|
|Max confidence|0.9982455968856812|

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
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 2088.` |
| 2 | `  +1.reserved = )<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 716.` |
| 3 | `  +1.reserved = ,<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.920. Support: 772.` |
| 4 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ ^1.roles in {DECLARATION} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 380.` |
| 5 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {,, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 876.` |
| 6 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {,, }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 249.` |
| 7 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 2<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ ^1.roles in {SCOPE} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 155.` |
| 8 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 2127.` |
| 9 | `  +1.reserved = )<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 722.` |
| 10 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {,}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 275.` |
| 11 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.923. Support: 239.` |
| 12 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.962. Support: 221.` |
| 13 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 2<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ ^1.roles in {BLOCK} and not in {DECLARATION, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.937. Support: 150.` |
| 14 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {BLOCK, DECLARATION, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 329.` |
| 15 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.951. Support: 2040.` |
| 16 | `  -1.reserved not in {(}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 285.` |
| 17 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.997. Support: 165.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles in {EXPRESSION, NAME}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 150.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles in {EXPRESSION} and not in {NAME}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 1909.` |
| 20 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.925. Support: 275.` |
| 21 | `  •••start_col ≥ 7<br>	∧ -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.956. Support: 262.` |
| 22 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 150.` |
| 23 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {COMMENT, EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression, ObjectExpression}<br>	∧ ^1.roles not in {SCOPE}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 401.` |
| 24 | `  -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 255.` |
| 25 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 199.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.internal_type not in {CommentLine}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.937. Support: 1907.` |
| 27 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {COMMENT}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 263.` |
| 28 | `  -1.diff_offset ≥ 2<br>	∧ -1.reserved not in {(, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {File, ObjectExpression}<br>	∧ ^1.roles in {BLOCK} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 161.` |
| 29 | `  •••start_col ≥ 33<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {File, ObjectExpression}<br>	∧ ^1.roles not in {BLOCK, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 250.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION, NAME}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 157.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION} and not in {NAME}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 1857.` |
| 32 | `  •••start_col ≥ 32<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {BlockStatement, ObjectExpression}<br>	∧ ^1.roles not in {FILE, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 278.` |
| 33 | `  •••start_col ≥ 32<br>	∧ -1.reserved not in {(, ,, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {BlockStatement, ObjectExpression}<br>	∧ ^1.roles in {OPERATOR} and not in {FILE, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 170.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION, MAP}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 2<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 152.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION} and not in {MAP}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 2<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 202.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 2<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 1690.` |
| 37 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {FILE} and not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.924. Support: 258.` |
| 38 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type = CommentLine<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles in {FILE} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 276.` |
| 39 | `  -1.reserved = (<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.932. Support: 833.` |
| 40 | `  -1.reserved not in {(}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 285.` |
| 41 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.998. Support: 201.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles in {EXPRESSION, NAME}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 127.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles in {EXPRESSION} and not in {NAME}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 1915.` |
| 44 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {COMMENT}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ ^1.roles in {FILE} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 285.` |
| 45 | `  •••start_col ≥ 7<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {BlockStatement, ObjectExpression}<br>	∧ ^1.roles not in {FILE, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 338.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles in {EXPRESSION} and not in {NAME}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 1710.` |
| 47 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {COMMENT}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles in {FILE} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 257.` |
| 48 | `  •••start_col ≥ 7<br>	∧ -1.diff_col ≥ 2<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {FILE, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 149.` |
| 49 | `  •••start_col ≥ 31<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {BlockStatement, ObjectExpression}<br>	∧ ^1.roles not in {FILE, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 240.` |
| 50 | `  +1.reserved = )<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 719.` |
| 51 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {), ,}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.920. Support: 320.` |
| 52 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.921. Support: 703.` |
| 53 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {,, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 890.` |
| 54 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {,, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 273.` |
| 55 | `  -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 263.` |
| 56 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 181.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION, MAP}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {DECLARATION} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 151.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION} and not in {MAP}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles in {DECLARATION} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 189.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 1712.` |
| 60 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 802.` |
| 61 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type = CommentLine<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ ^1.roles in {FILE} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 271.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.704918032786885, "max_conf": 0.9982455968856812, "max_support": 2127, "min_conf": 0.9203125238418579, "min_support": 127, "num_rules": 61}}
```
</details>
