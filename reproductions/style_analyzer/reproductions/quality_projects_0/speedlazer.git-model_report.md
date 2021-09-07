# Model report for file:///tmp/top-repos-quality-repos-1qqw5wjb/speedlazer.git HEAD 7876dce85b5f5706b475267cd03976f4a65d3755

### Dump

```json
{'created_at': '2021-09-01 20:52:10',
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
 'size': '22.3 kB',
 'tags': [],
 'uuid': '5438a75f-16e0-47a2-8ddb-b2c8e3224293',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-1qqw5wjb/speedlazer.git 7876dce85b5f5706b475267cd03976f4a65d3755

# javascript
282 rules, avg.len. 8.5
## train
PPCR: 0.988466
### report
macro
{'f1-score': 0.7453684112235657,
 'precision': 0.7900108976993694,
 'recall': 0.7193623524502306,
 'support': 105155}
micro
{'f1-score': 0.9378916837050069,
 'precision': 0.9378916837050069,
 'recall': 0.9378916837050069,
 'support': 105155}
weighted
{'f1-score': 0.9357841182873246,
 'precision': 0.9357048368603219,
 'recall': 0.9378916837050069,
 'support': 105155}
### report_full
macro
{'f1-score': 0.7304497756418833,
 'precision': 0.7900108976993694,
 'recall': 0.6983496398469258,
 'support': 106382}
micro
{'f1-score': 0.9324515333015029,
 'precision': 0.9378916837050069,
 'recall': 0.9270741290819876,
 'support': 106382}
weighted
{'f1-score': 0.9293906674360042,
 'precision': 0.935092272710784,
 'recall': 0.9270741290819876,
 'support': 106382}
## test
PPCR: 0.990729
### report
macro
{'f1-score': 0.7507673348821248,
 'precision': 0.8028462853836749,
 'recall': 0.7227617386449733,
 'support': 25646}
micro
{'f1-score': 0.9457225298292131,
 'precision': 0.9457225298292131,
 'recall': 0.9457225298292131,
 'support': 25646}
weighted
{'f1-score': 0.943860914116338,
 'precision': 0.9443459853590416,
 'recall': 0.9457225298292131,
 'support': 25646}
### report_full
macro
{'f1-score': 0.738108534093632,
 'precision': 0.8028462853836749,
 'recall': 0.7056162960484942,
 'support': 25886}
micro
{'f1-score': 0.9413180159900645,
 'precision': 0.9457225298292131,
 'recall': 0.9369543382523372,
 'support': 25886}
weighted
{'f1-score': 0.9386661747816751,
 'precision': 0.9439274118449672,
 'recall': 0.9369543382523372,
 'support': 25886}
```

## javascript
### Summary
149 rules, avg.len. 8.5

| | |
|-|-|
|Min support|140|
|Max support|22571|
|Min confidence|0.920121967792511|
|Max confidence|0.9998226165771484|

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
| 1 | `  -1.reserved = :<br>⇒ y = ␣<br>Confidence: 0.995. Support: 5798.` |
| 2 | `  •••start_col ≤ 25<br>	∧ -1.reserved = ,<br>	∧ +1.roles in {MAP} and not in {NAME}<br>	∧ +1.length ≥ 3<br>⇒ y = ⏎<br>Confidence: 0.946. Support: 1482.` |
| 3 | `  -1.reserved = ,<br>	∧ -2.reserved not in {}}<br>	∧ -5.length ≤ 8<br>	∧ +1.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.923. Support: 2058.` |
| 4 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,, :}<br>⇒ y = "<br>Confidence: 1.000. Support: 2819.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, [}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.951. Support: 461.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.951. Support: 2832.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 240.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {)}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 2183.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles in {RIGHT}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 1126.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {INCOMPLETE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 241.` |
| 11 | `  •••start_col ≥ 45<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.955. Support: 276.` |
| 12 | `  •••start_col ≤ 44<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.944. Support: 2067.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {DECLARATION, FUNCTION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 888.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ -5.diff_line ≤ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved = ><br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 179.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ -5.diff_line ≤ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles in {DECLARATION, FUNCTION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 221.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ -3.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 2386.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = import<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.internal_type = File<br>⇒ y = ⏎<br>Confidence: 0.970. Support: 215.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 751.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 234.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, :, ;, const, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 220.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, const, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 1012.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 441.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ -1.length ≥ 3<br>	∧ -3.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 483.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -3.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 1536.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type = JSXAttribute<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 160.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 278.` |
| 27 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -3.reserved not in {;}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.957. Support: 152.` |
| 28 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if}<br>	∧ -3.reserved not in {;}<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.987. Support: 189.` |
| 29 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 19075.` |
| 30 | `  -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 248.` |
| 31 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles in {EXPRESSION, INCOMPLETE} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 149.` |
| 32 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 14443.` |
| 33 | `  •••start_col ≥ 34<br>	∧ -1.reserved = ,<br>	∧ -2.roles in {MAP}<br>	∧ +1.roles in {MAP}<br>	∧ +1.length ≥ 3<br>⇒ y = ␣<br>Confidence: 0.982. Support: 195.` |
| 34 | `  -1.reserved = ,<br>	∧ -3.label in {<newline>}<br>	∧ +1.roles not in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.924. Support: 323.` |
| 35 | `  -1.reserved = ,<br>	∧ -3.label not in {<newline>}<br>	∧ -5.length ≤ 8<br>	∧ +1.roles not in {MAP}<br>	∧ +2.roles not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 1690.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_col ≥ 5<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.961. Support: 479.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.990. Support: 2618.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.reserved = import<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 170.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.label in {<space>}<br>	∧ -3.reserved not in {import}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +4.reserved not in {}}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.939. Support: 2195.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.973. Support: 987.` |
| 41 | `  •••start_col ≤ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = File<br>⇒ y = ⏎⏎<br>Confidence: 0.960. Support: 164.` |
| 42 | `  -1.diff_offset ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {DECLARATION, FUNCTION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 882.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = ><br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 161.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles in {DECLARATION, FUNCTION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 492.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {DECLARATION, FUNCTION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 200.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 174.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, :, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.937. Support: 2950.` |
| 48 | `  •••start_col ≤ 22<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.927. Support: 1107.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 207.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 200.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, const, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>	∧ ^2.roles not in {INITIALIZATION}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 976.` |
| 52 | `  -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ -3.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 544.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -3.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.928. Support: 1525.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.internal_type = JSXOpeningElement<br>⇒ y = ∅<br>Confidence: 0.971. Support: 188.` |
| 55 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 15671.` |
| 56 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {:}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.997. Support: 176.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), :}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 11071.` |
| 58 | `  -1.reserved = from<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 290.` |
| 59 | `  -1.reserved not in {,, :, from}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.953. Support: 2684.` |
| 60 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 2186.` |
| 61 | `  -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {INCOMPLETE} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.932. Support: 213.` |
| 62 | `  -1.reserved = {<br>	∧ -3.diff_col ≤ 7<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ +4.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.976. Support: 1288.` |
| 63 | `  •••start_col ≥ 9<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = import<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved not in {=}<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.986. Support: 243.` |
| 64 | `  •••start_col ≤ 5<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.942. Support: 231.` |
| 65 | `  •••start_col ≤ 25<br>	∧ -1.reserved not in {(, :, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.930. Support: 1496.` |
| 66 | `  -1.reserved not in {(, :, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = ObjectProperty<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 408.` |
| 67 | `  •••start_col ≤ 5<br>	∧ -1.reserved not in {(, :, ;, {}<br>	∧ -3.diff_col = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {ObjectProperty}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 158.` |
| 68 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {:}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 1.000. Support: 2773.` |
| 69 | `  •••start_col ≥ 45<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.968. Support: 300.` |
| 70 | `  •••start_col ≤ 44<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.955. Support: 2066.` |
| 71 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = JSXAttribute<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 178.` |
| 72 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 2470.` |
| 73 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.reserved = }<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.955. Support: 325.` |
| 74 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.reserved = ]<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.948. Support: 223.` |
| 75 | `  •••start_col ≤ 18<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.reserved not in {}}<br>	∧ -5.length ≤ 4<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 142.` |
| 76 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 222.` |
| 77 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 328.` |
| 78 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, :}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {INCOMPLETE} and not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 223.` |
| 79 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, :}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ +4.roles not in {MAP}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 1641.` |
| 80 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 584.` |
| 81 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = Identifier<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 223.` |
| 82 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.952. Support: 157.` |
| 83 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, if}<br>	∧ -5.reserved = const<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.924. Support: 164.` |
| 84 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if}<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved = ]<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.986. Support: 178.` |
| 85 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -5.reserved not in {const}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved not in {=, [, ], {, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ +3.reserved = =<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 273.` |
| 86 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if}<br>	∧ -5.reserved not in {const}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved not in {=, [, ], {, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ +3.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 22571.` |
| 87 | `  •••start_col ≤ 25<br>	∧ -1.reserved = ,<br>	∧ +1.roles in {MAP}<br>	∧ +1.length ≥ 3<br>⇒ y = ⏎<br>Confidence: 0.947. Support: 1438.` |
| 88 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 223.` |
| 89 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {)}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 145.` |
| 90 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, :}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 4892.` |
| 91 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = from<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.998. Support: 281.` |
| 92 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = "<br>Confidence: 0.991. Support: 2477.` |
| 93 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {IMPORT}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 149.` |
| 94 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {IMPORT}<br>	∧ +4.reserved not in {}}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.933. Support: 2202.` |
| 95 | `  •••start_col ≤ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {MODULE} and not in {OPERATOR}<br>⇒ y = ⏎⏎<br>Confidence: 0.956. Support: 149.` |
| 96 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {DECLARATION, FUNCTION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 869.` |
| 97 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ -5.diff_line ≤ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 177.` |
| 98 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ -5.diff_line ≤ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.roles in {DECLARATION, FUNCTION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 242.` |
| 99 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 179.` |
| 100 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, :, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.934. Support: 2690.` |
| 101 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -3.reserved not in {;}<br>	∧ +1.reserved not in {{, }}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.966. Support: 161.` |
| 102 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.internal_type = JSXOpeningElement<br>⇒ y = ∅<br>Confidence: 0.978. Support: 162.` |
| 103 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 242.` |
| 104 | `  -1.reserved = ,<br>	∧ +1.roles in {MAP}<br>	∧ +1.length ≤ 2<br>	∧ +4.reserved = }<br>⇒ y = ␣<br>Confidence: 0.936. Support: 304.` |
| 105 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 2307.` |
| 106 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.label in {<space>}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 1793.` |
| 107 | `  •••start_col ≤ 44<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.951. Support: 2010.` |
| 108 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {INCOMPLETE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 249.` |
| 109 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.diff_col ≤ 7<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {INCOMPLETE}<br>	∧ ^2.roles in {LIST}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 222.` |
| 110 | `  •••start_col ≤ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = Program<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎⏎<br>Confidence: 0.986. Support: 173.` |
| 111 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 2392.` |
| 112 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -3.length ≤ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.939. Support: 140.` |
| 113 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if}<br>	∧ -2.length ≥ 3<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.972. Support: 160.` |
| 114 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, =, const, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 15215.` |
| 115 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 572.` |
| 116 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, :, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 520.` |
| 117 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, :, {}<br>	∧ -3.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 388.` |
| 118 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, :, {}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 212.` |
| 119 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, :, {}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 187.` |
| 120 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, :, {}<br>	∧ -3.diff_line ≥ 1<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 176.` |
| 121 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, :, {}<br>	∧ -3.diff_line = 0<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 3967.` |
| 122 | `  •••start_col ≤ 10<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.internal_type = File<br>⇒ y = ⏎⏎<br>Confidence: 0.979. Support: 165.` |
| 123 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type = JSXAttribute<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 175.` |
| 124 | `  •••start_col ≤ 32<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.982. Support: 191.` |
| 125 | `  -1.reserved = ,<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.roles not in {FUNCTION}<br>⇒ y = ⏎<br>Confidence: 0.961. Support: 942.` |
| 126 | `  -1.reserved = ,<br>	∧ -4.diff_line = 0<br>	∧ +1.roles in {NUMBER}<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 438.` |
| 127 | `  -1.reserved = ,<br>	∧ -4.diff_line = 0<br>	∧ +1.length ≤ 2<br>	∧ +4.reserved = }<br>	∧ ^1.roles in {LITERAL}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 244.` |
| 128 | `  •••start_col ≤ 27<br>	∧ -1.reserved = ,<br>	∧ -4.diff_line = 0<br>	∧ +1.roles not in {NUMBER}<br>	∧ ^1.roles in {LITERAL}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ⏎<br>Confidence: 0.943. Support: 643.` |
| 129 | `  -1.internal_type = Identifier<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -2.label not in {<space>}<br>	∧ -5.diff_line ≤ 1<br>	∧ +1.reserved = =<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 180.` |
| 130 | `  -1.internal_type = Identifier<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -2.label not in {<space>}<br>	∧ -5.diff_line ≤ 1<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.roles in {DECLARATION, FUNCTION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 237.` |
| 131 | `  •••start_col ≤ 25<br>	∧ -1.reserved = ,<br>	∧ +1.roles not in {NAME}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = ObjectExpression<br>⇒ y = ⏎<br>Confidence: 0.947. Support: 1412.` |
| 132 | `  -1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved = }<br>	∧ ^1.internal_type = ObjectExpression<br>⇒ y = ␣<br>Confidence: 0.943. Support: 235.` |
| 133 | `  -1.reserved = ,<br>	∧ -4.diff_line ≥ 1<br>	∧ ^1.internal_type not in {ObjectExpression}<br>⇒ y = ⏎<br>Confidence: 0.943. Support: 427.` |
| 134 | `  -1.reserved = ,<br>	∧ -2.roles in {EXPRESSION}<br>	∧ -4.diff_line = 0<br>	∧ ^1.internal_type not in {ObjectExpression}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 1477.` |
| 135 | `  -1.reserved = ,<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ -4.diff_line = 0<br>	∧ -5.diff_col ≥ 8<br>	∧ -5.length ≤ 1<br>	∧ ^1.internal_type not in {ObjectExpression}<br>⇒ y = ␣<br>Confidence: 0.921. Support: 714.` |
| 136 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.label in {<space>}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {)}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 1751.` |
| 137 | `  -1.internal_type = Identifier<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -2.label in {<space>}<br>	∧ -4.roles in {FUNCTION}<br>	∧ ^1.roles in {DECLARATION, FUNCTION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 156.` |
| 138 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {DECLARATION, FUNCTION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 871.` |
| 139 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ -2.label not in {<space>}<br>	∧ -5.diff_line ≤ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = ><br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 150.` |
| 140 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 15423.` |
| 141 | `  -1.reserved = ,<br>	∧ +1.roles in {NUMBER}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 394.` |
| 142 | `  •••start_col ≤ 27<br>	∧ -1.reserved = ,<br>	∧ -2.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved = }<br>	∧ ^1.roles in {MAP}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 191.` |
| 143 | `  -1.reserved = ,<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ -3.roles not in {STRING}<br>	∧ +1.roles not in {NAME, NUMBER}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.968. Support: 1314.` |
| 144 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_col ≤ 7<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +4.reserved not in {}}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>	∧ ^2.roles not in {LIST}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.920. Support: 2460.` |
| 145 | `  •••start_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles in {FILE}<br>⇒ y = ⏎⏎<br>Confidence: 0.968. Support: 174.` |
| 146 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if}<br>	∧ -2.length ≥ 3<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.975. Support: 178.` |
| 147 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles in {RIGHT} and not in {STRING}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 1126.` |
| 148 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = import<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles in {FILE}<br>⇒ y = ⏎<br>Confidence: 0.993. Support: 218.` |
| 149 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, const, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.internal_type not in {ObjectProperty}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 908.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.523489932885907, "max_conf": 0.9998226165771484, "max_support": 22571, "min_conf": 0.920121967792511, "min_support": 140, "num_rules": 149}}
```
</details>
