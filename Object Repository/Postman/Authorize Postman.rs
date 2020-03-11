<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Authorize Postman</name>
   <tag></tag>
   <elementGuidId>f708252c-7d6f-4c3e-b716-a9f36542106d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;username\&quot;: \&quot;${ert_username}\&quot;,\n    \&quot;password\&quot;: \&quot;${decoded_password}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/auth/gsso/login/password</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>4bd2524f-101d-4ced-be70-3a5e85bdf097</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.ert_username</defaultValue>
      <description></description>
      <id>f217f3cc-4645-4733-b2f2-464783b5a830</id>
      <masked>false</masked>
      <name>ert_username</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.decoded_password</defaultValue>
      <description></description>
      <id>faf53141-5e20-4c90-83bd-03840d31a687</id>
      <masked>false</masked>
      <name>decoded_password</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
