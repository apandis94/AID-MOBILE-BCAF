<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>919. PPKD - Rule Maksimum InaturaProgram</name>
   <tag></tag>
   <elementGuidId>2f7885bd-3b7c-4508-9702-4f8d06ec132c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;NoKontrak\&quot;: \&quot;${accno}\&quot;,\n  \&quot;Year\&quot;: \&quot;${year}\&quot;,\n  \&quot;InaturaProgramID\&quot;: ${id_inatura},\n  \&quot;TotalKomisiDibagikan\&quot;: 400000.0,\n  \&quot;TotalKomisiDicadangkan\&quot;: 50000.0\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>5e18a4ba-c0b4-412b-950a-4ad204a1ad6d</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic QkFTSUNfQVVUSDpNb2JpbGUxMjMr</value>
      <webElementGuid>28c99211-1c89-41a9-9870-958c4b1df59f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://192.168.1.254:8089/api/PermohonanPembayaranKomisiDealer/GetFormulaRuleMaksimum/InaturaProgram</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'9530708518-001'</defaultValue>
      <description></description>
      <id>c9d4f108-5a91-4863-93bd-cc4153badcaf</id>
      <masked>false</masked>
      <name>accno</name>
   </variables>
   <variables>
      <defaultValue>'2022'</defaultValue>
      <description></description>
      <id>652d7ded-d987-4924-a96e-a3520c50654c</id>
      <masked>false</masked>
      <name>year</name>
   </variables>
   <variables>
      <defaultValue>3047</defaultValue>
      <description></description>
      <id>c56fef32-d842-46bd-8b92-2b76a00dd4fc</id>
      <masked>false</masked>
      <name>id_inatura</name>
   </variables>
   <variables>
      <defaultValue>400000.0</defaultValue>
      <description></description>
      <id>08017e29-923d-4f0d-a71b-ee07b7752c20</id>
      <masked>false</masked>
      <name>komisi_bag</name>
   </variables>
   <variables>
      <defaultValue>50000.0</defaultValue>
      <description></description>
      <id>9922447b-682b-4b54-b36a-2cc3af84fd8b</id>
      <masked>false</masked>
      <name>komisi_cad</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
