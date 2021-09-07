# Model report for file:///tmp/top-repos-quality-repos-u039w1z_/daily-algorithms.git HEAD c8a92a2708924cbb7733fd3c5bbc2745192ad7fe

### Dump

```json
{'created_at': '2021-09-02 00:37:48',
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
 'size': '18.4 kB',
 'tags': [],
 'uuid': 'ad5d746a-979f-48e0-ae25-92f51712eaed',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-u039w1z_/daily-algorithms.git c8a92a2708924cbb7733fd3c5bbc2745192ad7fe

# javascript
143 rules, avg.len. 7.4
## train
PPCR: 0.979521
### report
macro
{'f1-score': 0.7532485657366401,
 'precision': 0.8129430523603495,
 'recall': 0.7312476728075394,
 'support': 28507}
micro
{'f1-score': 0.9386817272950503,
 'precision': 0.9386817272950503,
 'recall': 0.9386817272950503,
 'support': 28507}
weighted
{'f1-score': 0.9340415202379079,
 'precision': 0.9326076348825157,
 'recall': 0.9386817272950503,
 'support': 28507}
### report_full
macro
{'f1-score': 0.7478339905989071,
 'precision': 0.8129430523603495,
 'recall': 0.720574258590927,
 'support': 29103}
micro
{'f1-score': 0.9289706648151363,
 'precision': 0.9386817272950503,
 'recall': 0.9194584750712985,
 'support': 29103}
weighted
{'f1-score': 0.9218161898921425,
 'precision': 0.92730381740619,
 'recall': 0.9194584750712985,
 'support': 29103}
## test
PPCR: 0.978010
### report
macro
{'f1-score': 0.7532320938206291,
 'precision': 0.824731446609631,
 'recall': 0.7272626192269888,
 'support': 7383}
micro
{'f1-score': 0.9376947040498442,
 'precision': 0.9376947040498442,
 'recall': 0.9376947040498442,
 'support': 7383}
weighted
{'f1-score': 0.9338737818539656,
 'precision': 0.9335813234679778,
 'recall': 0.9376947040498442,
 'support': 7383}
### report_full
macro
{'f1-score': 0.7473526766841284,
 'precision': 0.824731446609631,
 'recall': 0.7158172483738146,
 'support': 7549}
micro
{'f1-score': 0.9272702919903563,
 'precision': 0.9376947040498442,
 'recall': 0.9170751092859981,
 'support': 7549}
weighted
{'f1-score': 0.920847984765606,
 'precision': 0.9281512276513426,
 'recall': 0.9170751092859981,
 'support': 7549}
```

## javascript
### Summary
103 rules, avg.len. 7.1

| | |
|-|-|
|Min support|145|
|Max support|5465|
|Min confidence|0.9209523797035217|
|Max confidence|0.9991243481636047|

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
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 3338.` |
| 2 | `  +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 238.` |
| 3 | `  -1.reserved = {<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.988. Support: 721.` |
| 4 | `  -1.reserved not in {{}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.970. Support: 712.` |
| 5 | `  -1.reserved not in {{}<br>	∧ -1.roles in {FUNCTION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.935. Support: 481.` |
| 6 | `  -1.reserved = (<br>	∧ -1.roles not in {FUNCTION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 303.` |
| 7 | `  -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 3012.` |
| 8 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = ForStatement<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 172.` |
| 9 | `  -1.diff_col ≥ 9<br>	∧ -1.internal_type = CommentBlock<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 393.` |
| 10 | `  -1.diff_col ≥ 9<br>	∧ -1.internal_type not in {CommentBlock}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.diff_offset ≥ 20<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.949. Support: 445.` |
| 11 | `  -1.diff_col ≤ 8<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.924. Support: 1332.` |
| 12 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 395.` |
| 13 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved = if<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 258.` |
| 14 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved not in {return, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 289.` |
| 15 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved not in {return, {, }}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 3134.` |
| 16 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 3269.` |
| 17 | `  +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.976. Support: 801.` |
| 18 | `  -1.reserved = {<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.994. Support: 750.` |
| 19 | `  -1.reserved not in {{}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 241.` |
| 20 | `  -1.reserved = ;<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, OPERATOR}<br>	∧ ^2.roles in {SCOPE}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 203.` |
| 21 | `  -1.reserved not in {;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.932. Support: 2965.` |
| 22 | `  -1.diff_col ≥ 9<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_line ≥ 1<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 399.` |
| 23 | `  -1.diff_col ≥ 9<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_line ≤ 1<br>	∧ -5.diff_offset ≥ 20<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.935. Support: 439.` |
| 24 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 428.` |
| 25 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.928. Support: 312.` |
| 26 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved = if<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 243.` |
| 27 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 283.` |
| 28 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved not in {{, }}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 2894.` |
| 29 | `  -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.999. Support: 521.` |
| 30 | `  -1.reserved = ;<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ␣<br>Confidence: 0.991. Support: 176.` |
| 31 | `  -1.reserved = (<br>	∧ -1.roles not in {FUNCTION}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 320.` |
| 32 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {FUNCTION}<br>	∧ -3.reserved not in {=}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 2179.` |
| 33 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {FUNCTION}<br>	∧ -3.reserved not in {=}<br>	∧ +1.internal_type = Identifier<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.921. Support: 272.` |
| 34 | `  •••start_col ≥ 5<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.length ≥ 3<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 180.` |
| 35 | `  -1.reserved not in {;, {}<br>	∧ -2.length ≤ 2<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 345.` |
| 36 | `  -1.reserved = (<br>	∧ -2.length ≤ 2<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 397.` |
| 37 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {;, {}<br>	∧ -2.length ≤ 2<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 5465.` |
| 38 | `  -1.reserved = (<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ∅<br>Confidence: 0.997. Support: 193.` |
| 39 | `  -1.reserved not in {(, {}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.963. Support: 1589.` |
| 40 | `  -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 3614.` |
| 41 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 1181.` |
| 42 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {FOR} and not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 159.` |
| 43 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {FOR, OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.939. Support: 189.` |
| 44 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 3<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 1115.` |
| 45 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 792.` |
| 46 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 551.` |
| 47 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≥ 4<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +3.length ≥ 6<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 545.` |
| 48 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -2.roles in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +3.length ≤ 5<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 167.` |
| 49 | `  +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 201.` |
| 50 | `  ^1.roles in {IDENTIFIER} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 3267.` |
| 51 | `  +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.970. Support: 776.` |
| 52 | `  -1.reserved = {<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.988. Support: 722.` |
| 53 | `  -1.reserved = ;<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 198.` |
| 54 | `  -1.reserved = (<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 197.` |
| 55 | `  -1.reserved not in {(, ;, {}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 1551.` |
| 56 | `  -1.reserved not in {;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 3610.` |
| 57 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 1236.` |
| 58 | `  •••start_col ≥ 4<br>	∧ -1.diff_col ≥ 9<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {BLOCK} and not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 396.` |
| 59 | `  •••start_col ≥ 4<br>	∧ -1.diff_col ≥ 9<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {BLOCK, EXPRESSION}<br>	∧ -5.diff_offset ≥ 20<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.951. Support: 435.` |
| 60 | `  •••start_col ≥ 4<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 571.` |
| 61 | `  •••start_col ≥ 4<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 293.` |
| 62 | `  •••start_col ≥ 4<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {FUNCTION} and not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.945. Support: 208.` |
| 63 | `  •••start_col ≥ 4<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {IF} and not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 277.` |
| 64 | `  •••start_col ≥ 4<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<newline>} and not in {<space>}<br>	∧ -2.reserved not in {(}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 1444.` |
| 65 | `  •••start_col ≤ 3<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 1134.` |
| 66 | `  +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.975. Support: 806.` |
| 67 | `  -1.reserved = {<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.986. Support: 700.` |
| 68 | `  -1.reserved = ;<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 180.` |
| 69 | `  -1.reserved = (<br>	∧ -2.length ≤ 2<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 381.` |
| 70 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {;, {}<br>	∧ -2.length ≤ 2<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 5305.` |
| 71 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 258.` |
| 72 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved not in {{, }}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.939. Support: 2839.` |
| 73 | `  -1.reserved = ;<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 190.` |
| 74 | `  -1.reserved = (<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 214.` |
| 75 | `  -1.reserved not in {(, ;, {}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 1485.` |
| 76 | `  -1.reserved not in {;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.926. Support: 3626.` |
| 77 | `  •••start_col ≥ 4<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 1140.` |
| 78 | `  •••start_col ≥ 4<br>	∧ -1.diff_col ≥ 9<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_line ≥ 1<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 400.` |
| 79 | `  •••start_col ≥ 4<br>	∧ -1.diff_col ≥ 9<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_line ≤ 1<br>	∧ -5.diff_offset ≥ 20<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.950. Support: 427.` |
| 80 | `  •••start_col ≥ 4<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {FUNCTION} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.951. Support: 193.` |
| 81 | `  •••start_col ≥ 4<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≥ 4<br>	∧ +1.reserved not in {), ;, }}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.924. Support: 2533.` |
| 82 | `  •••start_col ≥ 4<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≤ 3<br>	∧ +1.reserved = {<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 414.` |
| 83 | `  •••start_col ≤ 3<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 1180.` |
| 84 | `  -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.999. Support: 544.` |
| 85 | `  -1.reserved = ;<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ␣<br>Confidence: 0.997. Support: 176.` |
| 86 | `  -1.reserved = {<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.989. Support: 743.` |
| 87 | `  -1.reserved not in {;, {}<br>	∧ -1.roles in {FUNCTION}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.935. Support: 518.` |
| 88 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {FUNCTION}<br>	∧ -3.reserved not in {=}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>	∧ ^2.roles in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 2015.` |
| 89 | `  -1.reserved not in {;, {}<br>	∧ -1.roles in {EXPRESSION, STRING}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = '<br>Confidence: 0.969. Support: 145.` |
| 90 | `  -1.reserved not in {;, {}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 4526.` |
| 91 | `  -1.diff_col ≥ 9<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_line ≥ 1<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 353.` |
| 92 | `  -1.diff_col ≥ 9<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_line ≤ 1<br>	∧ -5.diff_offset ≥ 20<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.939. Support: 438.` |
| 93 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 419.` |
| 94 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved = if<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 270.` |
| 95 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved not in {,, ;, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.997. Support: 190.` |
| 96 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.990. Support: 3271.` |
| 97 | `  +1.reserved = ]<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 243.` |
| 98 | `  -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.999. Support: 509.` |
| 99 | `  -1.reserved = ;<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ␣<br>Confidence: 0.988. Support: 204.` |
| 100 | `  -1.reserved = {<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.985. Support: 683.` |
| 101 | `  •••start_col ≥ 5<br>	∧ -1.reserved = (<br>	∧ -2.length ≥ 3<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.921. Support: 525.` |
| 102 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = (<br>	∧ -2.length ≤ 2<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 389.` |
| 103 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {;, {}<br>	∧ -2.length ≤ 2<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 319.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.135922330097087, "max_conf": 0.9991243481636047, "max_support": 5465, "min_conf": 0.9209523797035217, "min_support": 145, "num_rules": 103}}
```
</details>
