<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Check API Health</name>
   <tag></tag>
   <elementGuidId>d53b78cf-69a1-4128-9b1f-cec3ad7555c1</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${host}/health</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>7d69e9a8-6680-48f3-a078-8d15aa3222ae</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


//
//String[] arrayResponse = [&quot;why&quot;, &quot;hello&quot;, &quot;there&quot;]
//String[] arrayExpect = [&quot;there&quot;, &quot;why&quot;, &quot;hello&quot;]
//assertThat(arrayResponse).containsOnly(&quot;there&quot;, &quot;hello&quot;, &quot;why&quot;)
//assertThat(arrayResponse).containsOnlyElementsOf(Arrays.asList(&quot;why&quot;, &quot;there&quot;, &quot;hello&quot;))
//
//assertThat(arrayResponse).containsExactly(&quot;why&quot;, &quot;hello&quot;, &quot;there&quot;)
//assertThat(arrayResponse).containsExactlyElementsOf(Arrays.asList(&quot;why&quot;, &quot;hello&quot;, &quot;there&quot;))
//
//assertThat(arrayResponse).containsSequence(&quot;why&quot;, &quot;hello&quot;)
//assertThat(arrayResponse).containsSubsequence(&quot;why&quot;, &quot;there&quot;)
//assertThat(arrayResponse).containsAnyOf(&quot;why&quot;, &quot;nothing&quot;, &quot;new&quot;)

//assertThat(arrayResponse).contains(&quot;hello&quot;, atIndex(1))</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
