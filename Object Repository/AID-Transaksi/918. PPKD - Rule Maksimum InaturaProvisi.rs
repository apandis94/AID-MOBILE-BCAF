<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>918. PPKD - Rule Maksimum InaturaProvisi</name>
   <tag></tag>
   <elementGuidId>0073e913-84ab-4733-a6f5-6ddc253f4352</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;NoKontrak\&quot;: \&quot;${accno}\&quot;,\n  \&quot;Year\&quot;: \&quot;${year}\&quot;,\n  \&quot;TotalKomisiDibagikan\&quot;: ${komisi_bag},\n  \&quot;TotalKomisiDicadangkan\&quot;: ${komisi_cad}\n}&quot;,
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
      <webElementGuid>324f7c07-ddf0-4b68-8b21-46aea0268ce1</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic QkFTSUNfQVVUSDpNb2JpbGUxMjMr</value>
      <webElementGuid>cd30318b-25c5-41e0-9c14-ccad97f8e261</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://192.168.1.254:8089/api/PermohonanPembayaranKomisiDealer/GetFormulaRuleMaksimum/InaturaProvisi</restUrl>
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
      <id>a26e90e9-6dd9-4f37-a542-8717a6cabdaf</id>
      <masked>false</masked>
      <name>accno</name>
   </variables>
   <variables>
      <defaultValue>'2022'</defaultValue>
      <description></description>
      <id>296c06fc-5983-4ae3-8832-152a37fdca83</id>
      <masked>false</masked>
      <name>year</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>85eb1ea5-f02d-4da3-b53a-5f303c6a85e7</id>
      <masked>false</masked>
      <name>komisi_bag</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>ab817c28-63d1-4609-b640-e2ea0be957d9</id>
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
