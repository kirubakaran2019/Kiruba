<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>li_Employee Name   Invalid</name>
   <tag></tag>
   <elementGuidId>daf9d443-66c2-4951-9533-cf20e38fab32</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//form[@id='frmLeaveApply']/fieldset/ol/li</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>li</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>Employee Name *
  

Invalid



        

            var employees_assignleave_txtEmployee = [{&quot;name&quot;:&quot;Linda Anderson&quot;,&quot;id&quot;:&quot;1&quot;},{&quot;name&quot;:&quot;Hannah Flores&quot;,&quot;id&quot;:&quot;2&quot;},{&quot;name&quot;:&quot;John Smith&quot;,&quot;id&quot;:&quot;3&quot;},{&quot;name&quot;:&quot;Steven Edwards&quot;,&quot;id&quot;:&quot;4&quot;},{&quot;name&quot;:&quot;Thomas Fleming&quot;,&quot;id&quot;:&quot;5&quot;},{&quot;name&quot;:&quot;Robert Craig&quot;,&quot;id&quot;:&quot;6&quot;},{&quot;name&quot;:&quot;Fiona Grace&quot;,&quot;id&quot;:&quot;7&quot;},{&quot;name&quot;:&quot;Russel Hamilton&quot;,&quot;id&quot;:&quot;8&quot;},{&quot;name&quot;:&quot;Jasmine Morgan&quot;,&quot;id&quot;:&quot;9&quot;}];

            $(document).ready(function() {
            
                var nameField = $(&quot;#assignleave_txtEmployee_empName&quot;);
                var idStoreField = $(&quot;#assignleave_txtEmployee_empId&quot;);
                var typeHint = 'Type for hints...';
                var hintClass = 'inputFormatHint';
                var loadingMethod = '';
                var loadingHint = 'Loading';
            
                if (idStoreField.val() != '') {
                    idStoreField.data('item.name', nameField.val());
                }
                
                nameField.data('typeHint', typeHint);
                nameField.data('loadingHint', loadingHint);
                
                nameField.one('focus', function() {

                        if ($(this).hasClass(hintClass)) {
                            $(this).val(&quot;&quot;);
                            $(this).removeClass(hintClass);
                        }

                    });
                    
                if( loadingMethod != 'ajax'){
                    if (nameField.val() == '' || nameField.val() == typeHint) {
                        nameField.val(typeHint).addClass(hintClass);
                    }

                    

                    nameField.autocomplete(employees_assignleave_txtEmployee, {

                        formatItem: function(item) {
                            return $('&lt;div/>').text(item.name).html();
                        },
                        formatResult: function(item) {
                            return item.name
                        }
                      ,matchContains:true
                        }).result(function(event, item) {
                            idStoreField.val(item.id);
                            idStoreField.data('item.name', item.name);
                        }

                    );
                 }else{
                        var value = nameField.val().trim();
                        nameField.val(loadingHint).addClass('ac_loading');
                        $.ajax({
                               url: &quot;/index.php/pim/getEmployeeListAjax&quot;,
                               data: '',
                               dataType: 'json',
                               success: function(employeeList){

                                     nameField.autocomplete(employeeList, {

                                                formatItem: function(item) {
                                                    return $('&lt;div/>').text(item.name).html();
                                                },
                                                formatResult: function(item) {
                                                    return item.name
                                                }
                                                
                                                ,matchContains:true
                                            }).result(function(event, item) {
                                                idStoreField.val(item.id);
                                                idStoreField.data('item.name', item.name);
                                            }

                                        );
                                         nameField.removeClass('ac_loading'); 
                                        
                                         if(value==''){
                                            nameField.val(typeHint).addClass(hintClass);
                                         } else {
                                            nameField.val(value).addClass();
                                         }
                                    }
                             });
                 }
                
            }); // End of $(document).ready

                 
        


</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;frmLeaveApply&quot;)/fieldset[1]/ol[1]/li[1]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//form[@id='frmLeaveApply']/fieldset/ol/li</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Assign Leave'])[2]/following::li[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Access Records'])[1]/following::li[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//ol/li</value>
   </webElementXpaths>
</WebElementEntity>
